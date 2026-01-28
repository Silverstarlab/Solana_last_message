import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import type { LastSignalRust } from "../target/types/last_signal_rust";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.LastSignalRust as anchor.Program<LastSignalRust>;

describe("Last Signal Rust", () => {
  const owner = program.provider.publicKey;
  const beneficiary = new pg.web3.Keypair();

  // Rust 방식 PDA 주소 찾기
  const [capsulePda] = pg.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("capsule"), owner.toBuffer()],
    program.programId
  );

  it("1. 캡슐 생성 (Create)", async () => {
    // Rust 함수: create_capsule -> createCapsule 로 호출
    const tx = await program.methods
      .createCapsule(beneficiary.publicKey, "Secret Message")
      .accounts({
        capsule: capsulePda,
        owner: owner,
        systemProgram: pg.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("생성 완료!");
  });

  it("2. 생존 신고 (Heartbeat)", async () => {
    const tx = await program.methods
      .heartbeat()
      .accounts({
        capsule: capsulePda,
        owner: owner,
      })
      .rpc();
    console.log("생존 신고 완료!");
  });
});
