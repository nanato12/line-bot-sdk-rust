package line.bot.generator;

import org.openapitools.codegen.*;
import org.openapitools.codegen.languages.RustClientCodegen;
import org.openapitools.codegen.model.ModelMap;
import org.openapitools.codegen.model.ModelsMap;

import java.io.File;
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

    private final Set<String> discriminatorChildren = new HashSet<>();
    private final Map<String, String> childDiscriminatorProp = new HashMap<>();
    private final Map<String, String> childDiscriminatorValue = new HashMap<>();

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
    public CodegenModel fromModel(String name,
            io.swagger.v3.oas.models.media.Schema schema) {
        CodegenModel model = super.fromModel(name, schema);

        if (schema.getDiscriminator() != null
                && schema.getDiscriminator().getPropertyName() != null
                && schema.getDiscriminator().getMapping() != null) {

            String discProp = schema.getDiscriminator().getPropertyName();
            Map<String, String> mapping = schema.getDiscriminator().getMapping();

            model.vendorExtensions.put("x-is-tagged-enum", true);
            model.vendorExtensions.put("x-discriminator-property", discProp);

            List<Map<String, String>> variants = new ArrayList<>();
            for (Map.Entry<String, String> entry : mapping.entrySet()) {
                String tagValue = entry.getKey();
                String ref = entry.getValue();
                String typeName = ref.contains("/")
                        ? ref.substring(ref.lastIndexOf('/') + 1)
                        : ref;

                String rustTypeName = toModelName(typeName);
                Map<String, String> variant = new HashMap<>();
                variant.put("tagValue", tagValue);
                variant.put("typeName", rustTypeName);
                variant.put("moduleName", toModelFilename(typeName));
                variant.put("variantName", rustTypeName);
                variants.add(variant);
            }
            model.vendorExtensions.put("x-enum-variants", variants);

            // Check if there are common fields (parent properties beyond discriminator)
            if (schema.getProperties() != null) {
                long commonFieldCount = schema.getProperties().keySet().stream()
                        .filter(p -> !p.equals(discProp))
                        .count();
                if (commonFieldCount > 0) {
                    model.vendorExtensions.put("x-has-common-fields", true);
                }
            }
        }

        return model;
    }

    @Override
    public Map<String, ModelsMap> postProcessAllModels(
            Map<String, ModelsMap> objs) {
        // First pass: collect all discriminator children
        for (Map.Entry<String, ModelsMap> entry : objs.entrySet()) {
            for (ModelMap mm : entry.getValue().getModels()) {
                CodegenModel model = mm.getModel();
                Object isTagged = model.vendorExtensions.get("x-is-tagged-enum");
                if (Boolean.TRUE.equals(isTagged)) {
                    String discProp = (String) model.vendorExtensions
                            .get("x-discriminator-property");
                    @SuppressWarnings("unchecked")
                    List<Map<String, String>> variants = (List<Map<String, String>>) model.vendorExtensions
                            .get("x-enum-variants");
                    if (variants != null) {
                        for (Map<String, String> v : variants) {
                            discriminatorChildren.add(v.get("typeName"));
                            childDiscriminatorProp.put(
                                    v.get("typeName"), discProp);
                            childDiscriminatorValue.put(
                                    v.get("typeName"), v.get("tagValue"));
                        }
                    }
                }
            }
        }

        // Second pass: remove discriminator property from child structs
        for (Map.Entry<String, ModelsMap> entry : objs.entrySet()) {
            for (ModelMap mm : entry.getValue().getModels()) {
                CodegenModel model = mm.getModel();
                if (discriminatorChildren.contains(model.classname)) {
                    String prop = childDiscriminatorProp.get(model.classname);
                    if (prop != null) {
                        model.vars.removeIf(v -> v.baseName.equals(prop));
                        model.allVars.removeIf(v -> v.baseName.equals(prop));
                        model.requiredVars.removeIf(
                                v -> v.baseName.equals(prop));
                        model.optionalVars.removeIf(
                                v -> v.baseName.equals(prop));
                    }
                    model.vendorExtensions.put(
                            "x-is-discriminator-child", true);
                    model.vendorExtensions.put(
                            "x-discriminator-property", prop);
                    model.vendorExtensions.put(
                            "x-discriminator-value",
                            childDiscriminatorValue.get(model.classname));
                }
            }
        }

        // Third pass: find which discriminator children are used as direct
        // field types (not just through their parent tagged enum).
        // These children need a serialization-only type field so the
        // discriminator value is present when the struct is serialized
        // outside of its parent enum (e.g., FlexBox in FlexBubble.body).
        Set<String> childrenUsedAsFields = new HashSet<>();
        for (Map.Entry<String, ModelsMap> entry : objs.entrySet()) {
            for (ModelMap mm : entry.getValue().getModels()) {
                CodegenModel model = mm.getModel();
                for (CodegenProperty var : model.vars) {
                    collectChildFieldRef(var, childrenUsedAsFields);
                    if (var.items != null) {
                        collectChildFieldRef(
                                var.items, childrenUsedAsFields);
                    }
                }
            }
        }
        for (Map.Entry<String, ModelsMap> entry : objs.entrySet()) {
            for (ModelMap mm : entry.getValue().getModels()) {
                CodegenModel model = mm.getModel();
                if (childrenUsedAsFields.contains(model.classname)) {
                    model.vendorExtensions.put(
                            "x-needs-type-field", true);
                }
            }
        }

        return super.postProcessAllModels(objs);
    }

    /**
     * If the property references a discriminator child model,
     * add its name to the result set.
     */
    private void collectChildFieldRef(
            CodegenProperty var, Set<String> result) {
        String ref = var.complexType;
        if (ref != null && discriminatorChildren.contains(ref)) {
            result.add(ref);
        }
    }
}
