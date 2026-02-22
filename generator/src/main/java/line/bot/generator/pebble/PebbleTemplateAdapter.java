package line.bot.generator.pebble;

import io.pebbletemplates.pebble.PebbleEngine;
import io.pebbletemplates.pebble.loader.Loader;
import io.pebbletemplates.pebble.template.PebbleTemplate;
import org.openapitools.codegen.api.TemplatingEngineAdapter;
import org.openapitools.codegen.api.TemplatingExecutor;

import java.io.*;
import java.util.Map;

public class PebbleTemplateAdapter implements TemplatingEngineAdapter {

    private final PebbleEngine engine;

    public PebbleTemplateAdapter() {
        this.engine = new PebbleEngine.Builder()
                .cacheActive(false)
                .newLineTrimming(false)
                .loader(new ResourceLoader())
                .autoEscaping(false)
                .extension(new LineBotPebbleExtension())
                .build();
    }

    @Override
    public String getIdentifier() {
        return "pebble";
    }

    @Override
    public String[] getFileExtensions() {
        return new String[]{"pebble"};
    }

    @Override
    public String compileTemplate(
            TemplatingExecutor executor,
            Map<String, Object> bundle,
            String templateFile) throws IOException {
        PebbleTemplate template = engine.getTemplate(templateFile);
        StringWriter writer = new StringWriter();
        template.evaluate(writer, bundle);
        return writer.toString();
    }

    /**
     * Custom loader that finds templates on the classpath using
     * the classloader that loaded this adapter class.
     */
    private static class ResourceLoader implements Loader<String> {

        private final ClassLoader cl = PebbleTemplateAdapter.class.getClassLoader();

        @Override
        public Reader getReader(String templateName) {
            InputStream is = cl.getResourceAsStream(templateName);
            if (is == null) {
                // Try thread context classloader as fallback
                is = Thread.currentThread().getContextClassLoader()
                        .getResourceAsStream(templateName);
            }
            if (is == null) {
                throw new RuntimeException(
                        "Could not find template: " + templateName);
            }
            return new InputStreamReader(is);
        }

        @Override
        public void setCharset(String charset) {
        }

        @Override
        public void setPrefix(String prefix) {
        }

        @Override
        public void setSuffix(String suffix) {
        }

        @Override
        public String resolveRelativePath(String relativePath, String anchorPath) {
            return relativePath;
        }

        @Override
        public String createCacheKey(String templateName) {
            return templateName;
        }

        @Override
        public boolean resourceExists(String templateName) {
            return cl.getResource(templateName) != null
                    || Thread.currentThread().getContextClassLoader()
                    .getResource(templateName) != null;
        }
    }
}
