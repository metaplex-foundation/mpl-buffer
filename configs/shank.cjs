const path = require("path");
const { generateIdl } = require("@metaplex-foundation/shank-js");

const idlDir = path.join(__dirname, "..", "idls");
const binaryInstallDir = path.join(__dirname, "..", ".crates");
const programDir = path.join(__dirname, "..", "programs");

generateIdl({
  generator: "shank",
  programName: "mpl_buffer_program",
  programId: "BUFFCb9CHNtEVhhTmZ8LBdzLgWQbvCxtcy7iNv3RZCLH",
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "mpl-buffer"),
});
