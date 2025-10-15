"use strict";
Object.defineProperty(exports, Symbol.toStringTag, { value: "Module" });
const routerUtils = require("@tanstack/router-utils");
const config = require("./config.cjs");
const routeHmrStatement = require("./route-hmr-statement.cjs");
const utils = require("./utils.cjs");
const unpluginRouterHmrFactory = (options = {}) => {
  let ROOT = process.cwd();
  let userConfig = options;
  return {
    name: "router-hmr-plugin",
    enforce: "pre",
    transform(code, id) {
      if (!code.includes("export const Route = createFileRoute(")) {
        return null;
      }
      if (utils.debug) console.info("Adding HMR handling to route ", id);
      const ast = routerUtils.parseAst({ code, filename: id, root: ROOT });
      ast.program.body.push(routeHmrStatement.routeHmrStatement);
      const result = routerUtils.generateFromAst(ast, {
        sourceMaps: true,
        filename: id,
        sourceFileName: id
      });
      if (utils.debug) {
        routerUtils.logDiff(code, result.code);
        console.log("Output:\n", result.code + "\n\n");
      }
      return result;
    },
    transformInclude(id) {
      return utils.fileIsInRoutesDirectory(id, userConfig.routesDirectory);
    },
    vite: {
      configResolved(config$1) {
        ROOT = config$1.root;
        config$1.mode;
        userConfig = config.getConfig(options, ROOT);
      }
    },
    rspack() {
      ROOT = process.cwd();
      userConfig = config.getConfig(options, ROOT);
    },
    webpack() {
      ROOT = process.cwd();
      userConfig = config.getConfig(options, ROOT);
    }
  };
};
exports.unpluginRouterHmrFactory = unpluginRouterHmrFactory;
//# sourceMappingURL=router-hmr-plugin.cjs.map
