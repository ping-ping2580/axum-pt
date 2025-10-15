import { parseAst, generateFromAst, logDiff } from "@tanstack/router-utils";
import { getConfig } from "./config.js";
import { routeHmrStatement } from "./route-hmr-statement.js";
import { fileIsInRoutesDirectory, debug } from "./utils.js";
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
      if (debug) console.info("Adding HMR handling to route ", id);
      const ast = parseAst({ code, filename: id, root: ROOT });
      ast.program.body.push(routeHmrStatement);
      const result = generateFromAst(ast, {
        sourceMaps: true,
        filename: id,
        sourceFileName: id
      });
      if (debug) {
        logDiff(code, result.code);
        console.log("Output:\n", result.code + "\n\n");
      }
      return result;
    },
    transformInclude(id) {
      return fileIsInRoutesDirectory(id, userConfig.routesDirectory);
    },
    vite: {
      configResolved(config) {
        ROOT = config.root;
        config.mode;
        userConfig = getConfig(options, ROOT);
      }
    },
    rspack() {
      ROOT = process.cwd();
      userConfig = getConfig(options, ROOT);
    },
    webpack() {
      ROOT = process.cwd();
      userConfig = getConfig(options, ROOT);
    }
  };
};
export {
  unpluginRouterHmrFactory
};
//# sourceMappingURL=router-hmr-plugin.js.map
