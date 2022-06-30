/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

package software.amazon.smithy.rust.codegen.smithy.protocols.parse

import software.amazon.smithy.model.shapes.BlobShape
import software.amazon.smithy.model.shapes.Shape
import software.amazon.smithy.model.shapes.TimestampShape
import software.amazon.smithy.rust.codegen.rustlang.Writable
import software.amazon.smithy.rust.codegen.rustlang.rust
import software.amazon.smithy.rust.codegen.rustlang.writable
import software.amazon.smithy.rust.codegen.smithy.RuntimeConfig
import software.amazon.smithy.rust.codegen.smithy.RuntimeType
import software.amazon.smithy.rust.codegen.smithy.RustSymbolProvider
import software.amazon.smithy.rust.codegen.smithy.rustType

class ParserUtil(private val symbolProvider: RustSymbolProvider, private val runtimeConfig: RuntimeConfig) {
    fun convertViaFrom(shape: Shape): Writable =
        writable {
            val oldSymbol = when (shape) {
                // TODO(understand what needs to be done for ByteStream)
                is BlobShape -> RuntimeType.Blob(runtimeConfig).toSymbol()
                is TimestampShape -> RuntimeType.DateTime(runtimeConfig).toSymbol()
                else -> symbolProvider.toSymbol(shape)
            }
            val newSymbol = symbolProvider.toSymbol(shape)
            if (oldSymbol.rustType() != newSymbol.rustType()) {
                rust(".map($newSymbol::from)")
            }
        }
}
