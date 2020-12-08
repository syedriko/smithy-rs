package software.amazon.smithy.rust.codegen.smithy.generators

import software.amazon.smithy.codegen.core.CodegenException
import software.amazon.smithy.model.knowledge.OperationIndex
import software.amazon.smithy.model.shapes.OperationShape
import software.amazon.smithy.model.shapes.StructureShape
import software.amazon.smithy.model.traits.ErrorTrait
import software.amazon.smithy.protocoltests.traits.HttpMessageTestCase
import software.amazon.smithy.protocoltests.traits.HttpRequestTestCase
import software.amazon.smithy.protocoltests.traits.HttpRequestTestsTrait
import software.amazon.smithy.protocoltests.traits.HttpResponseTestCase
import software.amazon.smithy.protocoltests.traits.HttpResponseTestsTrait
import software.amazon.smithy.rust.codegen.lang.Custom
import software.amazon.smithy.rust.codegen.lang.RustMetadata
import software.amazon.smithy.rust.codegen.lang.RustWriter
import software.amazon.smithy.rust.codegen.lang.escape
import software.amazon.smithy.rust.codegen.lang.rust
import software.amazon.smithy.rust.codegen.lang.rustBlock
import software.amazon.smithy.rust.codegen.lang.withBlock
import software.amazon.smithy.rust.codegen.smithy.RuntimeType
import software.amazon.smithy.rust.codegen.util.dq
import software.amazon.smithy.rust.codegen.util.inputShape
import software.amazon.smithy.rust.codegen.util.orNull
import software.amazon.smithy.rust.codegen.util.outputShape
import java.util.logging.Logger

data class ProtocolSupport(
    val requestBodySerialization: Boolean,
    val responseDeserialization: Boolean,
    val errorDeserialization: Boolean
)

/**
 * Generate protocol tests for an operation
 */
class HttpProtocolTestGenerator(
    private val protocolConfig: ProtocolConfig,
    private val protocolSupport: ProtocolSupport,
    private val operationShape: OperationShape,
    private val writer: RustWriter
) {
    private val logger = Logger.getLogger(javaClass.name)

    // TODO: remove these once Smithy publishes fixes.
    // These tests are not even attempted to be compiled
    val DisableTests = setOf(
        // This test is flaky because of set ordering serialization https://github.com/awslabs/smithy-rs/issues/37
        "AwsJson11Enums"
    )

    // These tests fail due to shortcomings in our implementation.
    // These could be configured via runtime configuration, but since this won't be long-lasting,
    // it makes sense to do the simplest thing for now.
    // The test will _fail_ if these pass, so we will discover & remove if we fix them by accident
    val ExpectFail = setOf(
        // Document support: https://github.com/awslabs/smithy-rs/issues/31
        "PutAndGetInlineDocumentsInput",
        "InlineDocumentInput",
        "InlineDocumentAsPayloadInput",

        // Query literals: https://github.com/awslabs/smithy-rs/issues/36
        "RestJsonConstantQueryString",
        "RestJsonConstantAndVariableQueryStringMissingOneValue",
        "RestJsonConstantAndVariableQueryStringAllValues",

        // Misc:
        "RestJsonQueryIdempotencyTokenAutoFill", // https://github.com/awslabs/smithy-rs/issues/34
        "RestJsonHttpPrefixHeadersArePresent" // https://github.com/awslabs/smithy-rs/issues/35
    )
    private val inputShape = operationShape.inputShape(protocolConfig.model)
    private val outputShape = operationShape.outputShape(protocolConfig.model)
    private val operationSymbol = protocolConfig.symbolProvider.toSymbol(operationShape)
    private val operationIndex = OperationIndex.of(protocolConfig.model)

    private val instantiator = with(protocolConfig) {
        Instantiator(symbolProvider, model, runtimeConfig)
    }

    sealed class TestCase {
        abstract val testCase: HttpMessageTestCase

        data class RequestTest(override val testCase: HttpRequestTestCase) : TestCase()
        data class ResponseTest(override val testCase: HttpResponseTestCase, val targetShape: StructureShape) :
            TestCase()
    }

    fun render() {
        val requestTests = operationShape.getTrait(HttpRequestTestsTrait::class.java)
            .orNull()?.testCases.orEmpty().map { TestCase.RequestTest(it) }
        val responseTests = operationShape.getTrait(HttpResponseTestsTrait::class.java)
            .orNull()?.testCases.orEmpty().map { TestCase.ResponseTest(it, outputShape) }

        val errorTests = operationIndex.getErrors(operationShape).flatMap { error ->
            val testCases = error.getTrait(HttpResponseTestsTrait::class.java).orNull()?.testCases.orEmpty()
            testCases.map { TestCase.ResponseTest(it, error) }
        }
        val allTests: List<TestCase> = (requestTests + responseTests + errorTests).filterMatching()
        if (allTests.isNotEmpty()) {
            val operationName = operationSymbol.name
            val testModuleName = "${operationName.toSnakeCase()}_request_test"
            val moduleMeta = RustMetadata(
                public = false,
                additionalAttributes = listOf(
                    Custom("cfg(test)"),
                    Custom("allow(unreachable_code, unused_variables)")
                )
            )
            writer.withModule(testModuleName, moduleMeta) {
                renderAllTestCases(allTests)
            }
        }
    }

    private fun RustWriter.renderAllTestCases(allTests: List<TestCase>) {
        allTests.forEach {
            renderTestCaseBlock(it.testCase, this) {
                when (it) {
                    is TestCase.RequestTest -> this.renderHttpRequestTestCase(it.testCase)
                    is TestCase.ResponseTest -> this.renderHttpResponseTestCase(it.testCase, it.targetShape)
                }
            }
        }
    }

    /**
     * Filter out test cases that are disabled or don't match the service protocol
     */
    private fun List<TestCase>.filterMatching(): List<TestCase> = this.filter { testCase ->
        testCase.testCase.protocol == protocolConfig.protocol &&
            !DisableTests.contains(testCase.testCase.id)
    }

    private fun renderTestCaseBlock(
        testCase: HttpMessageTestCase,
        testModuleWriter: RustWriter,
        block: RustWriter.() -> Unit
    ) {
        testModuleWriter.setNewlinePrefix("/// ")
        testCase.documentation.map {
            testModuleWriter.writeWithNoFormatting(it)
        }
        testModuleWriter.write("Test ID: ${testCase.id}")
        testModuleWriter.setNewlinePrefix("")
        testModuleWriter.writeWithNoFormatting("#[test]")
        if (ExpectFail.contains(testCase.id)) {
            testModuleWriter.writeWithNoFormatting("#[should_panic]")
        }
        val fnName = when (testCase) {
            is HttpResponseTestCase -> "_response"
            is HttpRequestTestCase -> "_request"
            else -> throw CodegenException("unknown test case type")
        }
        testModuleWriter.rustBlock("fn test_${testCase.id.toSnakeCase()}$fnName()") {
            block(this)
        }
    }

    private fun RustWriter.renderHttpRequestTestCase(
        httpRequestTestCase: HttpRequestTestCase
    ) {
        writeInline("let input =")
        instantiator.render(this, inputShape, httpRequestTestCase.params)
        write(";")
        if (protocolSupport.requestBodySerialization) {
            write("let http_request = ${protocolConfig.symbolProvider.toSymbol(inputShape).name}::assemble(input.request_builder_base(), input.build_body());")
        } else {
            write("let http_request = ${protocolConfig.symbolProvider.toSymbol(inputShape).name}::assemble(input.request_builder_base(), vec![]);")
        }
        with(httpRequestTestCase) {
            write(
                """
                    assert_eq!(http_request.method(), ${method.dq()});
                    assert_eq!(http_request.uri().path(), ${uri.dq()});
                """
            )
        }
        checkQueryParams(this, httpRequestTestCase.queryParams)
        checkForbidQueryParams(this, httpRequestTestCase.forbidQueryParams)
        checkRequiredQueryParams(this, httpRequestTestCase.requireQueryParams)
        checkHeaders(this, httpRequestTestCase.headers)
        checkForbidHeaders(this, httpRequestTestCase.forbidHeaders)
        checkRequiredHeaders(this, httpRequestTestCase.requireHeaders)
        if (protocolSupport.requestBodySerialization) {
            checkBody(this, httpRequestTestCase.body.orElse(""), httpRequestTestCase.bodyMediaType.orElse(null))
        }

        // Explicitly warn if the test case defined parameters that we aren't doing anything with
        with(httpRequestTestCase) {
            if (authScheme.isPresent) {
                logger.warning("Test case provided authScheme but this was ignored")
            }
            if (!httpRequestTestCase.vendorParams.isEmpty) {
                logger.warning("Test case provided vendorParams but these were ignored")
            }
        }
    }

    private fun RustWriter.renderHttpResponseTestCase(
        httpResponseTestCase: HttpResponseTestCase,
        expectedShape: StructureShape
    ) {
        if (!protocolSupport.responseDeserialization || (
            !protocolSupport.errorDeserialization && expectedShape.hasTrait(
                    ErrorTrait::class.java
                )
            )
        ) {
            rust("/* test case disabled for this protocol (not yet supported) */")
            if (ExpectFail.contains(httpResponseTestCase.id)) {
                // this test needs to fail, minor hack. Caused by overlap between ids of request & response tests
                write("todo!()")
            }
            return
        }
        writeInline("let expected_output =")
        instantiator.render(this, expectedShape, httpResponseTestCase.params)
        write(";")
        write("let http_response = #T::new()", RuntimeType.HttpResponseBuilder)
        httpResponseTestCase.headers.forEach { (key, value) ->
            writeWithNoFormatting(".header(${key.dq()}, ${value.dq()})")
        }
        rust(
            """
                .status(${httpResponseTestCase.code})
                .body(${httpResponseTestCase.body.orNull()?.dq()?.replace("#", "##") ?: "vec![]"})
                .unwrap();
            """
        )
        write("let parsed = #T::from_response(http_response);", operationSymbol)
        if (expectedShape.hasTrait(ErrorTrait::class.java)) {
            val errorSymbol = operationShape.errorSymbol(protocolConfig.symbolProvider)
            val errorVariant = protocolConfig.symbolProvider.toSymbol(expectedShape).name
            rustBlock("if let Err(#T::$errorVariant(actual_error)) = parsed", errorSymbol) {
                write("assert_eq!(expected_output, actual_error);")
            }
            rustBlock("else") {
                write("panic!(\"wrong variant: {:?}\", parsed);")
            }
        } else {
            write("assert_eq!(parsed.unwrap(), expected_output);")
        }
    }

    private fun checkRequiredHeaders(rustWriter: RustWriter, requireHeaders: List<String>) {
        basicCheck(requireHeaders, rustWriter, "required_headers", "require_headers")
    }

    private fun checkForbidHeaders(rustWriter: RustWriter, forbidHeaders: List<String>) {
        basicCheck(forbidHeaders, rustWriter, "forbidden_headers", "forbid_headers")
    }

    private fun checkBody(rustWriter: RustWriter, body: String, mediaType: String?) {
        if (body == "") {
            rustWriter.write("// No body")
            rustWriter.write("assert!(input.build_body().is_empty());")
        } else {
            // When we generate a body instead of a stub, drop the trailing `;` and enable the assertion
            assertOk(rustWriter) {
                rustWriter.write(
                    "#T(input.build_body(), ${
                    rustWriter.escape(body).dq()
                    }, #T::from(${(mediaType ?: "unknown").dq()}))",
                    RuntimeType.ProtocolTestHelper(protocolConfig.runtimeConfig, "validate_body"),
                    RuntimeType.ProtocolTestHelper(protocolConfig.runtimeConfig, "MediaType")
                )
            }
        }
    }

    private fun checkHeaders(rustWriter: RustWriter, headers: Map<String, String>) {
        if (headers.isEmpty()) {
            return
        }
        val variableName = "expected_headers"
        rustWriter.withBlock("let $variableName = &[", "];") {
            write(
                headers.entries.joinToString(",") {
                    "(${it.key.dq()}, ${it.value.dq()})"
                }
            )
        }
        assertOk(rustWriter) {
            write(
                "#T(&http_request, $variableName)",
                RuntimeType.ProtocolTestHelper(protocolConfig.runtimeConfig, "validate_headers")
            )
        }
    }

    private fun checkRequiredQueryParams(
        rustWriter: RustWriter,
        requiredParams: List<String>
    ) = basicCheck(requiredParams, rustWriter, "required_params", "require_query_params")

    private fun checkForbidQueryParams(
        rustWriter: RustWriter,
        forbidParams: List<String>
    ) = basicCheck(forbidParams, rustWriter, "forbid_params", "forbid_query_params")

    private fun checkQueryParams(
        rustWriter: RustWriter,
        queryParams: List<String>
    ) = basicCheck(queryParams, rustWriter, "expected_query_params", "validate_query_string")

    private fun basicCheck(
        params: List<String>,
        rustWriter: RustWriter,
        variableName: String,
        checkFunction: String
    ) {
        if (params.isEmpty()) {
            return
        }
        rustWriter.withBlock("let $variableName = ", ";") {
            strSlice(this, params)
        }
        assertOk(rustWriter) {
            write(
                "#T(&http_request, $variableName)",
                RuntimeType.ProtocolTestHelper(protocolConfig.runtimeConfig, checkFunction)
            )
        }
    }

    /**
     * wraps `inner` in a call to `protocol_test_helpers::assert_ok`, a convenience wrapper
     * for pretty prettying protocol test helper results
     */
    private fun assertOk(rustWriter: RustWriter, inner: RustWriter.() -> Unit) {
        rustWriter.write("#T(", RuntimeType.ProtocolTestHelper(protocolConfig.runtimeConfig, "assert_ok"))
        inner(rustWriter)
        rustWriter.write(");")
    }

    private fun strSlice(writer: RustWriter, args: List<String>) {
        writer.withBlock("&[", "]") {
            write(args.joinToString(",") { it.dq() })
        }
    }
}