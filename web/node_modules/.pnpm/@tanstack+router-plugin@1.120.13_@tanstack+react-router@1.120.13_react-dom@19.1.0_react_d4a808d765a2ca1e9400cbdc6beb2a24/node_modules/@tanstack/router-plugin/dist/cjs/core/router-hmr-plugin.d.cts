import { Config } from './config.cjs';
import { UnpluginFactory } from 'unplugin';
/**
 * This plugin adds HMR support for file routes.
 * It is only added to the composed plugin in dev when autoCodeSplitting is disabled, since the code splitting plugin
 * handles HMR for code-split routes itself.
 */
export declare const unpluginRouterHmrFactory: UnpluginFactory<Partial<Config> | undefined>;
