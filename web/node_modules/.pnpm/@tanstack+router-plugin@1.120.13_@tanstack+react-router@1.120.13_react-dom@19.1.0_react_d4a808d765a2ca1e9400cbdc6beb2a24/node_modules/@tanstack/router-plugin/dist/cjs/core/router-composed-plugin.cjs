"use strict";
Object.defineProperty(exports, Symbol.toStringTag, { value: "Module" });
const routerGeneratorPlugin = require("./router-generator-plugin.cjs");
const routerCodeSplitterPlugin = require("./router-code-splitter-plugin.cjs");
const routerHmrPlugin = require("./router-hmr-plugin.cjs");
const unpluginRouterComposedFactory = (options = {}, meta) => {
  const getPlugin = (pluginFactory) => {
    const plugin = pluginFactory(options, meta);
    if (!Array.isArray(plugin)) {
      return [plugin];
    }
    return plugin;
  };
  const routerGenerator = getPlugin(routerGeneratorPlugin.unpluginRouterGeneratorFactory);
  const routerCodeSplitter = getPlugin(routerCodeSplitterPlugin.unpluginRouterCodeSplitterFactory);
  const result = [...routerGenerator, ...routerCodeSplitter];
  const isProduction = process.env.NODE_ENV === "production";
  if (!isProduction && !options.autoCodeSplitting) {
    const routerHmr = getPlugin(routerHmrPlugin.unpluginRouterHmrFactory);
    result.push(...routerHmr);
  }
  return result;
};
exports.unpluginRouterComposedFactory = unpluginRouterComposedFactory;
//# sourceMappingURL=router-composed-plugin.cjs.map
