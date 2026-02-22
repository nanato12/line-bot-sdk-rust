package line.bot.generator;

import org.openapitools.codegen.*;
import org.openapitools.codegen.languages.RustClientCodegen;
import org.openapitools.codegen.model.ModelMap;
import org.openapitools.codegen.model.ModelsMap;
import org.openapitools.codegen.model.OperationMap;
import org.openapitools.codegen.model.OperationsMap;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;

public class LineBotRustGenerator extends RustClientCodegen {

    public static final String GENERATOR_NAME = "line-bot-sdk-rust-generator";

    private static final String LICENSE_HEADER = """
            /*
            * Copyright (C) 2016 LINE Corp.
            *
            * Licensed under the Apache License, Version 2.0 (the "License");
            * you may not use this file except in compliance with the License.
            * You may obtain a copy of the License at
            *
            *     http://www.apache.org/licenses/LICENSE-2.0
            *
            * Unless required by applicable law or agreed to in writing, software
            * distributed under the License is distributed on an "AS IS" BASIS,
            * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
            * See the License for the specific language governing permissions and
            * limitations under the License.
            */
            """;

    public LineBotRustGenerator() {
        super();
        setLibrary("hyper");
        embeddedTemplateDir = templateDir = GENERATOR_NAME;

        // Replace stock Mustache templates with Pebble templates
        modelTemplateFiles.clear();
        modelTemplateFiles.put(GENERATOR_NAME + "/model.pebble", ".rs");

        apiTemplateFiles.clear();
        apiTemplateFiles.put(GENERATOR_NAME + "/api.pebble", ".rs");

        supportingFiles.clear();
    }

    @Override
    public String getName() {
        return GENERATOR_NAME;
    }

    @Override
    public String getHelp() {
        return "LINE Bot SDK Rust generator with Pebble templates";
    }

    @Override
    public CodegenType getTag() {
        return CodegenType.OTHER;
    }

    @Override
    public void processOpts() {
        super.processOpts();

        // Re-clear template files after super.processOpts() which re-adds stock templates
        modelTemplateFiles.clear();
        modelTemplateFiles.put(GENERATOR_NAME + "/model.pebble", ".rs");
        apiTemplateFiles.clear();
        apiTemplateFiles.put(GENERATOR_NAME + "/api.pebble", ".rs");

        // Disable doc generation (stock templates use Mustache which we don't provide)
        modelDocTemplateFiles.clear();
        apiDocTemplateFiles.clear();

        // Read existing version from Cargo.toml
        String outputDir = getOutputDir();
        Path cargoToml = Path.of(outputDir, "Cargo.toml");
        if (Files.exists(cargoToml)) {
            try {
                for (String line : Files.readAllLines(cargoToml)) {
                    if (line.startsWith("version = \"")) {
                        String ver = line.replace("version = \"", "")
                                .replace("\"", "").trim();
                        additionalProperties.put("packageVersion", ver);
                        break;
                    }
                }
            } catch (Exception ignored) {
            }
        }

        additionalProperties.put("licenseHeader", LICENSE_HEADER);
        additionalProperties.put("httpUserAgent", "LINE-Bot-SDK-Rust/1");

        // Register supporting files (generated once per package)
        supportingFiles.clear();
        supportingFiles.add(new SupportingFile(
                GENERATOR_NAME + "/Cargo.toml.pebble", "", "Cargo.toml"));
        supportingFiles.add(new SupportingFile(
                GENERATOR_NAME + "/lib.pebble", "src", "lib.rs"));
        supportingFiles.add(new SupportingFile(
                GENERATOR_NAME + "/model_mod.pebble", "src/models", "mod.rs"));
        supportingFiles.add(new SupportingFile(
                GENERATOR_NAME + "/api_mod.pebble", "src/apis", "mod.rs"));
        supportingFiles.add(new SupportingFile(
                GENERATOR_NAME + "/configuration.pebble", "src/apis", "configuration.rs"));
        supportingFiles.add(new SupportingFile(
                GENERATOR_NAME + "/request.pebble", "src/apis", "request.rs"));
        supportingFiles.add(new SupportingFile(
                GENERATOR_NAME + "/client.pebble", "src/apis", "client.rs"));
    }

    @Override
    public Map<String, ModelsMap> postProcessAllModels(
            Map<String, ModelsMap> objs) {
        // Detect feature flags needed for Cargo.toml
        boolean needsSerdeWith = false;
        boolean needsUuid = false;
        for (Map.Entry<String, ModelsMap> entry : objs.entrySet()) {
            for (ModelMap mm : entry.getValue().getModels()) {
                CodegenModel model = mm.getModel();
                for (CodegenProperty var : model.vars) {
                    if (var.isNullable && !var.required) {
                        needsSerdeWith = true;
                    }
                    if ("uuid::Uuid".equals(var.dataType)) {
                        needsUuid = true;
                    }
                }
            }
        }
        additionalProperties.put("needsSerdeWith", needsSerdeWith);
        additionalProperties.put("needsUuid", needsUuid);

        return super.postProcessAllModels(objs);
    }

    @Override
    public OperationsMap postProcessOperationsWithModels(
            OperationsMap objs, List<ModelMap> allModels) {
        OperationMap ops = objs.getOperations();
        if (ops != null) {
            for (CodegenOperation op : ops.getOperation()) {
                for (CodegenParameter param : op.allParams) {
                    if (param.isUuid) {
                        additionalProperties.put("needsUuid", true);
                    }
                }
            }
        }
        return super.postProcessOperationsWithModels(objs, allModels);
    }
}
