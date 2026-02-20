package line.bot.generator.pebble;

import io.pebbletemplates.pebble.extension.AbstractExtension;
import io.pebbletemplates.pebble.extension.Filter;
import io.pebbletemplates.pebble.extension.Function;

import java.util.HashMap;
import java.util.Map;

public class LineBotPebbleExtension extends AbstractExtension {

    @Override
    public Map<String, Filter> getFilters() {
        return new HashMap<>();
    }

    @Override
    public Map<String, Function> getFunctions() {
        return new HashMap<>();
    }
}
