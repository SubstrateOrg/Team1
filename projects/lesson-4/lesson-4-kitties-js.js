// All code is wrapped within an async closure,
// allowing access to api, hashing, keyring, types, util.
// (async ({ api, hashing, keyring, types, util }) => {
//   ... any user code is executed here ...
// })();

const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

const nextKittyId = await api.query.kitties.nextKittyId();
console.log("Next Kitty Id is:", nextKittyId)

async function list_user_kitties(user) {
// list all user kitties
  current = await api.query.kitties.kittyItems([user, null])
  if(current.isSome) {
    current = current.unwrap()
    do {
      current = await api.query.kitties.kittyItems([user, current.prev])
      if(current.isNone)
        break
      console.log(current)
      current = current.unwrap()
      const kitty = await api.query.kitties.kitties(current.id.unwrap().toNumber())
      console.log(kitty)
    } while (current.prev.isSome)
  }
}

console.log("Kitties owned by ALICE")
await list_user_kitties(ALICE)

console.log("Kitties owned by BOB")
await list_user_kitties(BOB)
