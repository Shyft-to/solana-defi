import { gql, GraphQLClient } from "graphql-request";

const endpoint = `https://programs.shyft.to/v0/graphql/?api_key=api`;

const graphQLClient = new GraphQLClient(endpoint, {
  method: `POST`,
  jsonSerializer: {
    parse: JSON.parse,
    stringify: JSON.stringify,
  },
});

export async function queryLpByAddress(address:string) {
  // You can cherry pick what fields you want
  const query = gql`
 query MyQuery($where: pump_Global_bool_exp = {}) {
  pump_Global(where: $where) {
    pubkey
    complete
    _updatedAt
    _lamports
    realSolReserves
    realTokenReserves
    tokenTotalSupply
    virtualSolReserves
    virtualTokenReserves
}`;

  const variables = {
    where: {
      pubkey: {
        _eq: address,
      },
    },
  };

  const info:any = await graphQLClient.request(query, variables);
  const pump_Global = info?.pump_Global[0];
  const sol =  pump_Global?._lamports;
  const _updatedAt = pump_Global?._updatedAt;
  const authority = pump_Global?.authority;
  const initialRealTokenReserve = pump_Global?.initialRealTokenReserves;
  const initialVirtualSolReserves = pump_Global?.initialVirtualSolReserves
  const initialVirtualTokenReserves = pump_Global?.initialVirtualTokenReserves;
  const initialized = pump_Global?.initialized;
  const totalSupply = pump_Global?.tokenTotalSupply;
  return {
    sol,
    _updatedAt,
    authority,
    initialRealTokenReserve,
    initialVirtualSolReserves,
    initialVirtualTokenReserves,
    initialized,
    totalSupply
  }
}
async function main() {
  const info = await queryLpByAddress('EXaYqQbQYPFK8XStMWmA3g2tmNwYHYuLVb3auEJfirvq');
  console.log(info);
}

main();
