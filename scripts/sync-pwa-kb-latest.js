const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");

const repoRoot = path.resolve(__dirname, "..");
const sourceFile = path.join(repoRoot, "samples", "pwa-kb-adapter-output.sample.json");

function parseArgs(argv) {
  const options = {
    pwaRoot: process.env.HOPE_WEB_PWA_ROOT || process.env.HOPE_PWA_ROOT || "",
    skipTests: false,
  };
  for (let index = 2; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--pwa-root") {
      options.pwaRoot = path.resolve(argv[++index]);
    } else if (arg === "--skip-tests") {
      options.skipTests = true;
    } else {
      throw new Error(`Unknown argument: ${arg}`);
    }
  }
  return options;
}

function defaultPwaRoots() {
  const userProfile = process.env.USERPROFILE || process.env.HOME || "";
  return [
    userProfile ? path.join(userProfile, "Documents", "New project", "hope-web-pwa-inspect") : "",
    path.join("E:", "codex", "hope-web-pwa"),
  ].filter(Boolean);
}

function resolvePwaRoot(optionRoot) {
  const candidates = [optionRoot, ...defaultPwaRoots()].filter(Boolean);
  const root = candidates.find((candidate) => fs.existsSync(path.join(candidate, "package.json")));
  if (!root) {
    throw new Error("PWA root not found. Pass --pwa-root or set HOPE_WEB_PWA_ROOT.");
  }
  return root;
}

function run(command, args, cwd) {
  const useShell = process.platform === "win32" && /\.cmd$/i.test(command);
  const result = useShell
    ? spawnSync([command, ...args].join(" "), { cwd, encoding: "utf8", shell: true })
    : spawnSync(command, args, { cwd, encoding: "utf8", shell: false });
  return {
    command: [command, ...args].join(" "),
    cwd,
    status: result.status,
    stdout: String(result.stdout || "").trim(),
    stderr: String(result.stderr || "").trim(),
    error: result.error ? { code: result.error.code, message: result.error.message } : null,
    passed: result.status === 0,
  };
}

function main() {
  const options = parseArgs(process.argv);
  const pwaRoot = resolvePwaRoot(options.pwaRoot);
  const targetFiles = [
    path.join(pwaRoot, "public", "kb", "latest.json"),
    path.join(pwaRoot, "dist", "kb", "latest.json"),
  ].filter((file, index, files) => files.indexOf(file) === index && (index === 0 || fs.existsSync(file)));
  const npmBin = process.platform === "win32" ? "npm.cmd" : "npm";

  const adapterCheck = run(process.execPath, ["scripts/build-pwa-kb-adapter-output.js", "--check"], repoRoot);
  if (!adapterCheck.passed) {
    console.error(JSON.stringify({ status: "failed", stage: "hope_kb_adapter_check", adapter_check: adapterCheck }, null, 2));
    process.exit(adapterCheck.status || 1);
  }

  targetFiles.forEach((targetFile) => {
    fs.mkdirSync(path.dirname(targetFile), { recursive: true });
    fs.copyFileSync(sourceFile, targetFile);
  });

  const pwaTest = options.skipTests
    ? { command: "skipped", cwd: pwaRoot, status: 0, stdout: "", stderr: "", passed: true }
    : run(npmBin, ["test", "--", "--run", "src/lib/kbAdapter.test.ts"], pwaRoot);

  const evidence = {
    status: pwaTest.passed ? "synced" : "failed",
    source: path.relative(repoRoot, sourceFile),
    targets: targetFiles.map((targetFile) => path.relative(pwaRoot, targetFile)),
    pwa_root: pwaRoot,
    adapter_check: adapterCheck,
    copy: {
      passed: true,
      files: targetFiles.map((targetFile) => ({ path: path.relative(pwaRoot, targetFile), bytes: fs.statSync(targetFile).size })),
    },
    pwa_test: pwaTest,
  };

  console.log(JSON.stringify(evidence, null, 2));
  if (!pwaTest.passed) process.exit(pwaTest.status || 1);
}

main();
