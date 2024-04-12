import { readFileSync, writeFileSync } from "fs";
import minimist from "minimist";

(async () => {
  const install = JSON.parse(
    String(readFileSync(process.cwd() + "/updater/install.json"))
  );
  const $args = minimist(process.argv.slice(2));
  const platforms = String($args.platforms).split(",");

  for (const [key, _] of Object.entries(install.platforms)) {
    if (!platforms.includes(key)) {
      delete install.platforms[key];
    }
  }

  writeFileSync(
    process.cwd() + "/updater/install.json",
    JSON.stringify(install, null, 2)
  );
})();
