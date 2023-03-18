import { createHash } from "crypto"
import { readdirSync, readFileSync } from "fs"
import { load } from "protobufjs"
import { getPath } from "./get-path"

const tag = "nightly"
const proc = Bun.spawnSync(["git", "rev-parse", "HEAD"])
let commitSha = proc.stdout.toString().trim()
let dateIso = new Date().toISOString()

let protoRoot = await load("releasing/version-info.proto")
let Binary = protoRoot.lookupType("Binary")

const builtBinaries = readdirSync(getPath(".releases")).filter(
    (file) => !file.startsWith("version")
)

let binaries = []

for (const name of builtBinaries) {
    const contents = readFileSync(getPath(`.releases/${name}`)).toString()
    let sha256 = createHash("sha256").update(contents).digest("hex")

    const payload = {
        name,
        sha256,
    }

    const binaryMessage = Binary.create(payload)
    binaries.push(binaryMessage)
}

let VersionInfo = protoRoot.lookupType("VersionInfo")
const payload = { tag, dateIso, commitSha, binaries }
const versionMessage = VersionInfo.create(payload)

const releaseTxtContents = [
    `Release: ${tag}`,
    `Date: ${dateIso}`,
    `Commit SHA: ${commitSha}`,
    "",
    "Binaries (sha256):",
    ...binaries.map((binary) => `${binary.name}: ${binary.sha256}`),
].join("\n")

await Bun.write(getPath(".releases/version.txt"), releaseTxtContents)
await Bun.write(
    getPath(".releases/version.json"),
    JSON.stringify(versionMessage.toJSON())
)
await Bun.write(
    getPath(".releases/version.pb"),
    VersionInfo.encode(versionMessage).finish()
)
