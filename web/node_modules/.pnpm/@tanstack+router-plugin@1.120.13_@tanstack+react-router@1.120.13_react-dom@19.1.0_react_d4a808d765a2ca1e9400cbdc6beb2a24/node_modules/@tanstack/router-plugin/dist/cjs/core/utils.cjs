"use strict";
Object.defineProperty(exports, Symbol.toStringTag, { value: "Module" });
const node_path = require("node:path");
const debug = process.env.TSR_VITE_DEBUG && ["true", "router-plugin"].includes(process.env.TSR_VITE_DEBUG);
function fileIsInRoutesDirectory(filePath, routesDirectory) {
  const routesDirectoryPath = node_path.isAbsolute(routesDirectory) ? routesDirectory : node_path.join(process.cwd(), routesDirectory);
  const path = node_path.normalize(filePath);
  return path.startsWith(routesDirectoryPath);
}
exports.debug = debug;
exports.fileIsInRoutesDirectory = fileIsInRoutesDirectory;
//# sourceMappingURL=utils.cjs.map
