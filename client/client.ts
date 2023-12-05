import { PublicKey, SystemProgram, TransactionSignature, Connection } from "@solana/web3.js";
import { AnchorProvider, Program, ProgramAccount, Wallet } from "@coral-xyz/anchor";
import { IDL } from "../../idl/IDL";

const PROGRAM_ID = new PublicKey("CuVwVRBD5pVvzmnKsaTmVjaL1aeRSHDEusnJ45TXAKXm");

export function getProgramInstance(connection: Connection, wallet?: Wallet): Program {
    let provider = { connection };
    if (wallet && wallet.publicKey) {
        provider = new AnchorProvider(
            connection,
            wallet,
            AnchorProvider.defaultOptions()
        );
    }
    return new Program(IDL, PROGRAM_ID, provider);
}

export function getProposals(pg: Program): Promise<ProgramAccount[]> {
    return pg.account.proposal.all();
}

export function getVotes(pg: Program): Promise<ProgramAccount[]> {
    return pg.account.vote.all();
}

export async function createProposal(pg: Program, program: string, name: string, group: string, subGroup: string): Promise<TransactionSignature> {
    let programPk = new PublicKey(program);
    const [proposalPDA, _] = await PublicKey.findProgramAddress(
        [pg.provider.publicKey.toBuffer(), programPk.toBuffer()],
        pg.programId
    );
    return pg.methods
        .createProposal(name, group, subGroup)
        .accounts({
            author: pg.provider.publicKey,
            program: programPk,
            proposal: proposalPDA,
            systemProgram: SystemProgram.programId
        })
        .rpc();
}

export function deleteProposal(pg: Program, proposal: string): Promise<TransactionSignature> {
    return pg.methods
        .deleteProposal()
        .accounts({
            signer: pg.provider.publicKey,
            proposal: new PublicKey(proposal)
        })
        .rpc();
}

export async function createVote(pg: Program, proposal: string, positive: boolean): Promise<TransactionSignature> {
    let proposalPk = new PublicKey(proposal);
    const [votePDA, _] = await PublicKey.findProgramAddress(
        [pg.provider.publicKey.toBuffer(), proposalPk.toBuffer()],
        pg.programId
    );
    return pg.methods
        .createVote(positive)
        .accounts({
            author: pg.provider.publicKey,
            proposal: proposalPk,
            vote: votePDA,
            systemProgram: SystemProgram.programId
        })
        .rpc();
}

export function changeVote(pg: Program, proposal: string, vote: string, positive: boolean): Promise<TransactionSignature> {
    return pg.methods
        .changeVote(positive)
        .accounts({
            author: pg.provider.publicKey,
            proposal: new PublicKey(proposal),
            vote: new PublicKey(vote)
        })
        .rpc();
}

export function deleteVote(pg: Program, proposal: string, vote: string): Promise<TransactionSignature> {
    return pg.methods
        .deleteVote()
        .accounts({
            signer: pg.provider.publicKey,
            proposal: new PublicKey(proposal),
            vote: new PublicKey(vote),
        })
        .rpc();
}
