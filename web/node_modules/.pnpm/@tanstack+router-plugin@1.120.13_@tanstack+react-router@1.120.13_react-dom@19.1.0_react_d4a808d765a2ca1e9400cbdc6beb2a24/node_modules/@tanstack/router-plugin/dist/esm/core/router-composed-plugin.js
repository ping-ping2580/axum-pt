import { unpluginRouterGeneratorFactory } from "./router-generator-plugin.js";
import { unpluginRouterCodeSplitterFactory } from "./router-code-splitter-plugin.js";
import { unpluginRouterHmrFactory } from "./router-hmr-plugin.js";
const unpluginRouterComposedFactory = (options = {}, meta) => {
  const getPlugin = (pluginFactory) => {
    const plugin = pluginFactory(options, meta);
    if (!Array.isArray(plugin)) {
      return [plugin];
    }
    return plugin;
  };
  const routerGenerator = getPlugin(unpluginRouterGeneratorFactory);
  const routerCodeSplitter = getPlugin(unpluginRouterCodeSplitterFactory);
  const result = [...routerGenerator, ...routerCodeSplitter];
  const isProduction = process.env.NODE_ENV === "production";
  if (!isProduction && !options.autoCodeSplitting) {
    const routerHmr = getPlugin(unpluginRouterHmrFactory);
    result.push(...routerHmr);
  }
  return result;
};
export {
  unpluginRouterComposedFactory
};
//# sourceMappingURL=router-composed-plugin.js.map
