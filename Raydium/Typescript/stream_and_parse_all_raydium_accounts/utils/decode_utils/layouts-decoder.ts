import { struct, publicKey, u64, blob, i64, f64 } from './marshmallow';

const AUTHORIZED_LAYOUT = struct<any>([
    publicKey('staker'),
    publicKey('withdrawer')
]);

const LOCKUP_LAYOUT = struct<any>([
    i64('unix_timestamp'),
    u64('epoch'),
    publicKey('custodian')
]);

const META_LAYOUT = struct<any>([
    u64('rentExemptReserve'),
    blob(64,'authorized'),
    blob(48,'lockup'),
]);

const DELEGATION_LAYOUT = struct<any>([
    publicKey('voter'),
    u64('stake'),
    u64('activationEpoch'),
    u64('deactivationEpoch'),
    f64('warmupCooldownRate')
]);

const STAKE_LAYOUT = struct<any>([
    blob(64, 'delegation'),
    u64('creditsObserved')
]);

const STAKE_ACCOUNT_LAYOUT = struct<any>([
    blob(4, 'discriminator'),
    blob(120,'meta'),
    blob(72,'stake'),
    blob(8, 'flags')
]);

const INITIALIZED_ACCOUNT_LAYOUT = struct<any>([
    blob(4, 'discriminator'),
    blob(120,'meta')
]);

const DISCIMINATOR_LAYOUT = struct<any>([
    blob(4, 'discriminator')
]);

export function getStakeAccountType(accountData: Buffer) {
    try {
        const slicedAccountData = accountData.slice(0, 4);
        const discriminator = DISCIMINATOR_LAYOUT.decode(slicedAccountData).discriminator;

        if (discriminator[0] === 0) {
            return 'UNINITIALIZED'
        }
        else if (discriminator[0] === 1) {
            return 'INITIALIZED'
        }
        else if (discriminator[0] === 2) {
            return 'STAKE'
        }
        else {
            return 'NOT_FOUND'
        }
    } catch (error) {
        console.log("Account Discriminator Error", error);
        return 'NOT_FOUND'
    }
}

export function decodeInitializedAccount(accountData: Buffer) {
    try {
        if(accountData.length < 124)
            throw new Error("Invalid account data length");

        const decodedInitializedAccount = INITIALIZED_ACCOUNT_LAYOUT.decode(accountData);
        const decodedMeta = META_LAYOUT.decode(decodedInitializedAccount.meta);
        const decodedLockup = LOCKUP_LAYOUT.decode(decodedMeta.lockup);
        const decodedAuth = AUTHORIZED_LAYOUT.decode(decodedMeta.authorized);

        const decodedInitializedAccountResponse = {
            meta: {
                rentExemptReserve: BigInt(decodedMeta.rentExemptReserve).toString(),
                authorized: {
                    staker: decodedAuth.staker?.toBase58(),
                    withdrawer: decodedAuth.withdrawer?.toBase58(),
                },
                lockup: {
                    unix_timestamp: BigInt(decodedLockup.unix_timestamp).toString(),
                    epoch: BigInt(decodedLockup.epoch).toString(),
                    custodian: decodedLockup.custodian?.toBase58(),
    
                }
            }
        };
        return decodedInitializedAccountResponse;
        
    } catch (error) {
        console.log(error);
        return "UNABLE_TO_PARSE";
    }
}

export function decodeStakeAccount(accountData: Buffer) {
    try {
        if(accountData.length < 196)
            throw new Error("Invalid account data length");

        const decodedStakeAccount = STAKE_ACCOUNT_LAYOUT.decode(accountData);
        const decodedMeta = META_LAYOUT.decode(decodedStakeAccount.meta);
        const decodedLockup = LOCKUP_LAYOUT.decode(decodedMeta.lockup);
        const decodedAuth = AUTHORIZED_LAYOUT.decode(decodedMeta.authorized);
        const decodedStake = STAKE_LAYOUT.decode(decodedStakeAccount.stake);
        const decodedDelegation = DELEGATION_LAYOUT.decode(decodedStake.delegation);

        const decodedStakeAccountResponse = {
            meta: {
                rentExemptReserve: BigInt(decodedMeta.rentExemptReserve).toString(),
                authorized: {
                    staker: decodedAuth.staker?.toBase58(),
                    withdrawer: decodedAuth.withdrawer?.toBase58(),
                },
                lockup: {
                    unix_timestamp: BigInt(decodedLockup.unix_timestamp).toString(),
                    epoch: BigInt(decodedLockup.epoch).toString(),
                    custodian: decodedLockup.custodian?.toBase58(),
                }
            },
            stake: {
                creditsObserved: BigInt(decodedStake.creditsObserved).toString(),
                delegation: {
                    voter: decodedDelegation.voter?.toBase58(),
                    stake: BigInt(decodedDelegation.stake).toString(),
                    activationEpoch: BigInt(decodedDelegation.activationEpoch).toString(),
                    deactivationEpoch: BigInt(decodedDelegation.deactivationEpoch).toString(),
                    warmupCooldownRate: decodedDelegation.warmupCooldownRate
                }
            }
        }
        return decodedStakeAccountResponse;
    } catch (error) {
        console.log(error);
        return "UNABLE_TO_PARSE";
    }
}
