package line.bot.generator.pebble;

import io.pebbletemplates.pebble.extension.AbstractExtension;
import io.pebbletemplates.pebble.extension.Filter;
import io.pebbletemplates.pebble.extension.Function;
import io.pebbletemplates.pebble.template.EvaluationContext;
import io.pebbletemplates.pebble.template.PebbleTemplate;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class LineBotPebbleExtension extends AbstractExtension {

    @Override
    public Map<String, Filter> getFilters() {
        Map<String, Filter> filters = new HashMap<>();
        filters.put("snakeCase", new SnakeCaseFilter());
        return filters;
    }

    @Override
    public Map<String, Function> getFunctions() {
        return new HashMap<>();
    }

    private static class SnakeCaseFilter implements Filter {
        @Override
        public List<String> getArgumentNames() {
            return null;
        }

        @Override
        public Object apply(Object input, Map<String, Object> args,
                PebbleTemplate self, EvaluationContext context,
                int lineNumber) {
            if (input == null) return null;
            String s = input.toString();
            StringBuilder sb = new StringBuilder();
            for (int i = 0; i < s.length(); i++) {
                char c = s.charAt(i);
                if (Character.isUpperCase(c)) {
                    if (i > 0) sb.append('_');
                    sb.append(Character.toLowerCase(c));
                } else {
                    sb.append(c);
                }
            }
            return sb.toString();
        }
    }
}
