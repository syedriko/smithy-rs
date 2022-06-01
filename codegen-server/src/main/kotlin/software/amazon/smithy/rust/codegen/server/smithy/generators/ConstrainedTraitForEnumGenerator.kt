/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package software.amazon.smithy.rust.codegen.server.smithy.generators

import software.amazon.smithy.model.Model
import software.amazon.smithy.model.shapes.StringShape
import software.amazon.smithy.model.traits.EnumTrait
import software.amazon.smithy.rust.codegen.rustlang.RustWriter
import software.amazon.smithy.rust.codegen.rustlang.rustTemplate
import software.amazon.smithy.rust.codegen.smithy.RuntimeType
import software.amazon.smithy.rust.codegen.smithy.RustSymbolProvider
import software.amazon.smithy.rust.codegen.smithy.makeMaybeConstrained
import software.amazon.smithy.rust.codegen.util.expectTrait

// TODO Unit tests
/**
 * [ConstrainedTraitForEnumGenerator] TODO Docs
 */
class ConstrainedTraitForEnumGenerator(
    val model: Model,
    val symbolProvider: RustSymbolProvider,
    val writer: RustWriter,
    val shape: StringShape
) {
    fun render() {
        shape.expectTrait<EnumTrait>()

        val symbol = symbolProvider.toSymbol(shape)
        val name = symbol.name
        val unconstrainedType = "String"

        writer.rustTemplate(
            """
            impl #{ConstrainedTrait} for $name  {
                type Unconstrained = $unconstrainedType;
            }
            
            impl From<$unconstrainedType> for #{MaybeConstrained} {
                fn from(value: $unconstrainedType) -> Self {
                    Self::Unconstrained(value)
                }
            }
            """,
            "ConstrainedTrait" to RuntimeType.ConstrainedTrait(),
            "MaybeConstrained" to symbol.makeMaybeConstrained(),
        )
    }
}
