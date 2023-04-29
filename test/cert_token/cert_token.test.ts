import { expect, use } from "chai"
import chaiAsPromised from "chai-as-promised"
import CertTokenFactory from "./typedContract/constructors/cert_token"
import CertToken from "./typedContract/contracts/cert_token"
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api"
import { KeyringPair } from "@polkadot/keyring/types"
import { IdBuilder } from "./typedContract/types-arguments/cert_token"

use(chaiAsPromised)

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944")
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" })

describe("cert token test", () => {
  let certTokenFactory: CertTokenFactory
  let api: ApiPromise
  let deployer: KeyringPair

  let contract: CertToken
  const tokenName = "CertToken"
  const tokenSymbol = "CRT"

  before(async function setup(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider })
    deployer = keyring.addFromUri("//Alice")

    certTokenFactory = new CertTokenFactory(api, deployer)

    contract = new CertToken(
      (await certTokenFactory.new([tokenName], [tokenSymbol])).address,
      deployer,
      api
    )
  })

  after(async function tearDown() {
    await api.disconnect()
  })

  it("should mint first token", async () => {
    const id = IdBuilder.U8(1)
    const owner = await contract.withSigner(deployer).query.getTokenOwner(id)
    expect(owner.value.ok?.unwrap()).to.eq(deployer.address)
  })
})
