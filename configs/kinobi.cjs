const path = require("path");
const k = require("@metaplex-foundation/kinobi");

// Paths.
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const kinobi = k.createFromIdls([path.join(idlDir, "mpl_buffer_program.json")]);

// Update programs.
kinobi.update(
  new k.updateProgramsVisitor({
    mplBufferProgram: { name: "mplBuffer" },
  })
);

// Update accounts.
kinobi.update(
  new k.updateAccountsVisitor({
    bufferMetadata: {
      seeds: [
        k.constantPdaSeedNodeFromString("Buffer"),
        k.programIdPdaSeedNode(),
        k.variablePdaSeedNode("buffer", k.publicKeyTypeNode(), "The address of the buffer account"),
      ],
    },
  })
);

// Update instructions.
kinobi.update(
  new k.updateInstructionsVisitor({
    create: {
      byteDeltas: [
        k.instructionByteDeltaNode(k.accountLinkNode("buffer")),
      ],
    },
  })
);

// Set ShankAccount discriminator.
const key = (name) => ({ field: "key", value: k.enumValueNode("Key", name) });
kinobi.update(
  new k.setAccountDiscriminatorFromFieldVisitor({
    myAccount: key("Uninitialized"),
    myPdaAccount: key("BufferMetadataAccount"),
  })
);

// Render JavaScript.
const jsDir = path.join(clientDir, "js", "src", "generated");
const prettier = require(path.join(clientDir, "js", ".prettierrc.json"));
kinobi.accept(new k.renderJavaScriptVisitor(jsDir, { prettier }));

// Render Rust.
const crateDir = path.join(clientDir, "rust");
const rustDir = path.join(clientDir, "rust", "src", "generated");
kinobi.accept(
  new k.renderRustVisitor(rustDir, {
    formatCode: true,
    crateFolder: crateDir,
  })
);
