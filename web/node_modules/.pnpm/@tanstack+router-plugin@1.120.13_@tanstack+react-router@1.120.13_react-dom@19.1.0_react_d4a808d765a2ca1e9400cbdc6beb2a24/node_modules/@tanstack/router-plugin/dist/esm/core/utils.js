import { isAbsolute, join, normalize } from "node:path";
const debug = process.env.TSR_VITE_DEBUG && ["true", "router-plugin"].includes(process.env.TSR_VITE_DEBUG);
function fileIsInRoutesDirectory(filePath, routesDirectory) {
  const routesDirectoryPath = isAbsolute(routesDirectory) ? routesDirectory : join(process.cwd(), routesDirectory);
  const path = normalize(filePath);
  return path.startsWith(routesDirectoryPath);
}
export {
  debug,
  fileIsInRoutesDirectory
};
//# sourceMappingURL=utils.js.map
