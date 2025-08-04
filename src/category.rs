use clap::ValueEnum;

#[derive(strum_macros::Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Category {
    #[strum(serialize = "4chan-themed")]
    FourchanThemed,
    #[strum(serialize = "8bit-chain-ecosystem")]
    EightBitChainEcosystem,
    #[strum(serialize = "aave-tokens")]
    AaveTokens,
    #[strum(serialize = "abstract-ecosystem")]
    AbstractEcosystem,
    #[strum(serialize = "account-abstraction")]
    AccountAbstraction,
    #[strum(serialize = "action-games")]
    ActionGames,
    #[strum(serialize = "adidas-ecosystem")]
    AdidasEcosystem,
    #[strum(serialize = "adventure-games")]
    AdventureGames,
    #[strum(serialize = "aelf-ecosystem")]
    AelfEcosystem,
    #[strum(serialize = "ai-agent-launchpad")]
    AIAgentLaunchpad,
    #[strum(serialize = "ai-agents")]
    AIAgents,
    #[strum(serialize = "ai-applications")]
    AIApplications,
    #[strum(serialize = "ai-framework")]
    AIFramework,
    #[strum(serialize = "ai-meme-coins")]
    AIMemeCoins,
    #[strum(serialize = "airdao-ecosystem")]
    AirdaoEcosystem,
    #[strum(serialize = "airdropped-tokens-by-nft-projects")]
    AirdroppedTokensByNFTProjects,
    #[strum(serialize = "alameda-research-portfolio")]
    AlamedaResearchPortfolio,
    #[strum(serialize = "alephium-ecosystem")]
    AlephiumEcosystem,
    #[strum(serialize = "algorand-ecosystem")]
    AlgorandEcosystem,
    #[strum(serialize = "algorithmic-stablecoin")]
    AlgorithmicStablecoin,
    #[strum(serialize = "alienx-ecosystem")]
    AlienXEcosystem,
    #[strum(serialize = "alleged-sec-securities")]
    AllegedSecSecurities,
    #[strum(serialize = "alveychain-ecosystem")]
    AlveychainEcosystem,
    #[strum(serialize = "analytics")]
    Analytics,
    #[strum(serialize = "ancient8-ecosystem")]
    Ancient8Ecosystem,
    #[strum(serialize = "andreessen-horowitz-a16z-portfolio")]
    AndreessenHorowitzA16zPortfolio,
    #[strum(serialize = "animal-racing")]
    AnimalRacing,
    #[strum(serialize = "anime-themed-coins")]
    AnimeThemedCoins,
    #[strum(serialize = "animoca-brands")]
    AnimocaBrands,
    #[strum(serialize = "apechain-ecosystem")]
    ApechainEcosystem,
    #[strum(serialize = "apex-chain-ecosystem")]
    ApexChainEcosystem,
    #[strum(serialize = "appchains")]
    Appchains,
    #[strum(serialize = "aptos-ecosystem")]
    AptosEcosystem,
    #[strum(serialize = "arbitrum-ecosystem")]
    ArbitrumEcosystem,
    #[strum(serialize = "arbitrum-nova-ecosystem")]
    ArbitrumNovaEcosystem,
    #[strum(serialize = "arcade-games")]
    ArcadeGames,
    #[strum(serialize = "archway-ecosystem")]
    ArchwayEcosystem,
    #[strum(serialize = "art")]
    Art,
    #[strum(serialize = "art-blocks-ecosystem")]
    ArtBlocksEcosystem,
    #[strum(serialize = "artela-ecosystem")]
    ArtelaEcosystem,
    #[strum(serialize = "artificial-intelligence")]
    ArtificialIntelligence,
    #[strum(serialize = "asc-20")]
    Asc20,
    #[strum(serialize = "asset-manager")]
    AssetManager,
    #[strum(serialize = "astar-ecosystem")]
    AstarEcosystem,
    #[strum(serialize = "astar-zkevm-ecosystem")]
    AstarZkevmEcosystem,
    #[strum(serialize = "atomicals-arc-20")]
    AtomicalsArc20,
    #[strum(serialize = "augmented-reality")]
    AugmentedReality,
    #[strum(serialize = "aurora-ecosystem")]
    AuroraEcosystem,
    #[strum(serialize = "automated-market-maker-amm")]
    AutomatedMarketMakerAmm,
    #[strum(serialize = "avalanche-ecosystem")]
    AvalancheEcosystem,
    #[strum(serialize = "avalanche-subnet")]
    AvalancheSubnet,
    #[strum(serialize = "axie-infinity")]
    AxieInfinity,
    #[strum(serialize = "azuki-ecosystem")]
    AzukiEcosystem,
    #[strum(serialize = "bahamut-ecosystem")]
    BahamutEcosystem,
    #[strum(serialize = "base-ecosystem")]
    BaseEcosystem,
    #[strum(serialize = "base-meme-coins")]
    BaseMemeCoins,
    #[strum(serialize = "beam-ecosystem")]
    BeamEcosystem,
    #[strum(serialize = "beamprivacy-ecosystem")]
    BeamprivacyEcosystem,
    #[strum(serialize = "believe-app-ecosystem")]
    BelieveAppEcosystem,
    #[strum(serialize = "berachain-ecosystem")]
    BerachainEcosystem,
    #[strum(serialize = "bevm-ecosystem")]
    BevmEcosystem,
    #[strum(serialize = "big-data")]
    BigData,
    #[strum(serialize = "binance-alpha-spotlight")]
    BinanceAlphaSpotlight,
    #[strum(serialize = "binance-hodler-airdrops")]
    BinanceHodlerAirdrops,
    #[strum(serialize = "binance-launchpad")]
    BinanceLaunchpad,
    #[strum(serialize = "binance-launchpool")]
    BinanceLaunchpool,
    #[strum(serialize = "binance-megadrop")]
    BinanceMegadrop,
    #[strum(serialize = "binance-peg-tokens")]
    BinancePegTokens,
    #[strum(serialize = "binance-wallet-ido")]
    BinanceWalletIdo,
    #[strum(serialize = "bitcichain-ecosystem")]
    BitcichainEcosystem,
    #[strum(serialize = "bitcoin-ecosystem")]
    BitcoinEcosystem,
    #[strum(serialize = "bitcoin-fork")]
    BitcoinFork,
    #[strum(serialize = "bitcoin-layer-2")]
    BitcoinLayer2,
    #[strum(serialize = "bitgert-ecosystem")]
    BitgertEcosystem,
    #[strum(serialize = "bitlayer-ecosystem")]
    BitlayerEcosystem,
    #[strum(serialize = "bitrock-ecosystem")]
    BitrockEcosystem,
    #[strum(serialize = "bitstarters-launchpad")]
    BitstartersLaunchpad,
    #[strum(serialize = "bittensor-ecosystem")]
    BittensorEcosystem,
    #[strum(serialize = "bittensor-subnets")]
    BittensorSubnets,
    #[strum(serialize = "bittorrent-ecosystem")]
    BittorrentEcosystem,
    #[strum(serialize = "blast-ecosystem")]
    BlastEcosystem,
    #[strum(serialize = "blockchain-capital-portfolio")]
    BlockchainCapitalPortfolio,
    #[strum(serialize = "binance-smart-chain")]
    BinanceSmartChain,
    #[strum(serialize = "boba-bnb-ecosystem")]
    BobaBnbEcosystem,
    #[strum(serialize = "boba-network-ecosystem")]
    BobaNetworkEcosystem,
    #[strum(serialize = "letsbonk-fun-ecosystem")]
    LetsbonkFunEcosystem,
    #[strum(serialize = "bored-ape-ecosystem")]
    BoredApeEcosystem,
    #[strum(serialize = "botanix-ecosystem")]
    BotanixEcosystem,
    #[strum(serialize = "bouncebit-ecosystem")]
    BouncebitEcosystem,
    #[strum(serialize = "brc-20")]
    Brc20,
    #[strum(serialize = "breeding")]
    Breeding,
    #[strum(serialize = "bridged-dai")]
    BridgedDai,
    #[strum(serialize = "bridged-frax")]
    BridgedFrax,
    #[strum(serialize = "bridged-stablecoins")]
    BridgedStablecoins,
    #[strum(serialize = "bridged-tokens")]
    BridgedTokens,
    #[strum(serialize = "bridged-usdc")]
    BridgedUsdc,
    #[strum(serialize = "bridged-usdt")]
    BridgedUsdt,
    #[strum(serialize = "bridged-wavax")]
    BridgedWavax,
    #[strum(serialize = "bridged-wbnb")]
    BridgedWbnb,
    #[strum(serialize = "bridged-wbtc")]
    BridgedWbtc,
    #[strum(serialize = "bridged-weth")]
    BridgedWeth,
    #[strum(serialize = "bridged-wsteth")]
    BridgedWsteth,
    #[strum(serialize = "bridge-governance-tokens")]
    BridgeGovernanceTokens,
    #[strum(serialize = "bsquared-network-ecosystem")]
    BsquaredNetworkEcosystem,
    #[strum(serialize = "btcfi")]
    Btcfi,
    #[strum(serialize = "business-services")]
    BusinessServices,
    #[strum(serialize = "camelot-launchpad")]
    CamelotLaunchpad,
    #[strum(serialize = "canto-ecosystem")]
    CantoEcosystem,
    #[strum(serialize = "capital-launchpad")]
    CapitalLaunchpad,
    #[strum(serialize = "cardano-ecosystem")]
    CardanoEcosystem,
    #[strum(serialize = "card-games")]
    CardGames,
    #[strum(serialize = "cat-themed-coins")]
    CatThemedCoins,
    #[strum(serialize = "cefi")]
    Cefi,
    #[strum(serialize = "celebrity-themed-coins")]
    CelebrityThemedCoins,
    #[strum(serialize = "celer-network")]
    CelerNetwork,
    #[strum(serialize = "celo-ecosystem")]
    CeloEcosystem,
    #[strum(serialize = "centralized-exchange-cex-product")]
    CentralizedExchangeCexProduct,
    #[strum(serialize = "centralized-exchange-token-cex")]
    CentralizedExchangeTokenCex,
    #[strum(serialize = "chain-abstraction")]
    ChainAbstraction,
    #[strum(serialize = "chaingpt-pad")]
    ChaingptPad,
    #[strum(serialize = "charity")]
    Charity,
    #[strum(serialize = "chiliz-ecosystem")]
    ChilizEcosystem,
    #[strum(serialize = "chinese-meme")]
    ChineseMeme,
    #[strum(serialize = "christmas-themed")]
    ChristmasThemed,
    #[strum(serialize = "chromia-ecosystem")]
    ChromiaEcosystem,
    #[strum(serialize = "circle-ventures-portfolio")]
    CircleVenturesPortfolio,
    #[strum(serialize = "clanker-ecosystem")]
    ClankerEcosystem,
    #[strum(serialize = "cny-stablecoin")]
    CnyStablecoin,
    #[strum(serialize = "coinbase-50-index")]
    Coinbase50Index,
    #[strum(serialize = "coinbase-ventures-portfolio")]
    CoinbaseVenturesPortfolio,
    #[strum(serialize = "collectibles")]
    Collectibles,
    #[strum(serialize = "combo-ecosystem")]
    ComboEcosystem,
    #[strum(serialize = "commodity-backed-stablecoin")]
    CommodityBackedStablecoin,
    #[strum(serialize = "communication")]
    Communication,
    #[strum(serialize = "compound-tokens")]
    CompoundTokens,
    #[strum(serialize = "conflux-ecosystem")]
    ConfluxEcosystem,
    #[strum(serialize = "consensys-portofolio")]
    ConsensysPortofolio,
    #[strum(serialize = "core-ecosystem")]
    CoreEcosystem,
    #[strum(serialize = "corn-ecosystem")]
    CornEcosystem,
    #[strum(serialize = "cosmos-ecosystem")]
    CosmosEcosystem,
    #[strum(serialize = "country-themed-meme-coins")]
    CountryThemedMemeCoins,
    #[strum(serialize = "creatorbid-ecosystem")]
    CreatorbidEcosystem,
    #[strum(serialize = "cronos-ecosystem")]
    CronosEcosystem,
    #[strum(serialize = "cronos-zkevm-ecosystem")]
    CronosZkevmEcosystem,
    #[strum(serialize = "cross-chain-communication")]
    CrossChainCommunication,
    #[strum(serialize = "crypto-backed-stablecoin")]
    CryptoBackedStablecoin,
    #[strum(serialize = "asset-backed-tokens")]
    AssetBackedTokens,
    #[strum(serialize = "ctokens")]
    Ctokens,
    #[strum(serialize = "curve-ecosystem")]
    CurveEcosystem,
    #[strum(serialize = "cyber-ecosystem")]
    CyberEcosystem,
    #[strum(serialize = "cyberkongz-ecosystem")]
    CyberkongzEcosystem,
    #[strum(serialize = "cybersecurity")]
    Cybersecurity,
    #[strum(serialize = "daomaker-ecosystem")]
    DaomakerEcosystem,
    #[strum(serialize = "daos-fun-ecosystem")]
    DaosFunEcosystem,
    #[strum(serialize = "data-availability")]
    DataAvailability,
    #[strum(serialize = "decentralized-exchange")]
    DecentralizedExchange,
    #[strum(serialize = "decentralized-finance-defi")]
    DecentralizedFinanceDefi,
    #[strum(serialize = "identity")]
    Identity,
    #[strum(serialize = "decentralized-lottery")]
    DecentralizedLottery,
    #[strum(serialize = "decentralized-science-desci")]
    DecentralizedScienceDesci,
    #[strum(serialize = "deepbrain-chain-ecosystem")]
    DeepbrainChainEcosystem,
    #[strum(serialize = "defai")]
    Defai,
    #[strum(serialize = "defiance-capital-portfolio")]
    DefianceCapitalPortfolio,
    #[strum(serialize = "defi-index")]
    DefiIndex,
    #[strum(serialize = "defimetachain-ecosystem")]
    DefimetachainEcosystem,
    #[strum(serialize = "defiverse-ecosystem")]
    DefiverseEcosystem,
    #[strum(serialize = "degen-ecosystem")]
    DegenEcosystem,
    #[strum(serialize = "delabs")]
    Delabs,
    #[strum(serialize = "delphi-ventures-portfolio")]
    DelphiVenturesPortfolio,
    #[strum(serialize = "depin")]
    Depin,
    #[strum(serialize = "decentralized-derivatives")]
    DecentralizedDerivatives,
    #[strum(serialize = "desci-meme")]
    DesciMeme,
    #[strum(serialize = "dex-aggregator")]
    DexAggregator,
    #[strum(serialize = "dfk-chain-ecosystem")]
    DfkChainEcosystem,
    #[strum(serialize = "dinari-ecosystem")]
    DinariEcosystem,
    #[strum(serialize = "directed-acyclic-graph-dag")]
    DirectedAcyclicGraphDag,
    #[strum(serialize = "discord-bots")]
    DiscordBots,
    #[strum(serialize = "dmex-ecosystem")]
    DmexEcosystem,
    #[strum(serialize = "dn-404")]
    Dn404,
    #[strum(serialize = "dogechain-ecosystem")]
    DogechainEcosystem,
    #[strum(serialize = "dog-themed-coins")]
    DogThemedCoins,
    #[strum(serialize = "doodles-llc")]
    DoodlesLlc,
    #[strum(serialize = "dragonfly-capital-portfolio")]
    DragonflyCapitalPortfolio,
    #[strum(serialize = "drc-20")]
    Drc20,
    #[strum(serialize = "duckchain-ecosystem")]
    DuckchainEcosystem,
    #[strum(serialize = "duck-themed-coins")]
    DuckThemedCoins,
    #[strum(serialize = "dwf-labs-portfolio")]
    DwfLabsPortfolio,
    #[strum(serialize = "eclipse-ecosystem")]
    EclipseEcosystem,
    #[strum(serialize = "e-commerce")]
    ECommerce,
    #[strum(serialize = "edgeware-ecosystem")]
    EdgewareEcosystem,
    #[strum(serialize = "education")]
    Education,
    #[strum(serialize = "edu-chain-ecosystem")]
    EduChainEcosystem,
    #[strum(serialize = "egirl-capital-portfolio")]
    EgirlCapitalPortfolio,
    #[strum(serialize = "elastos-smart-contract-chain-ecosystem")]
    ElastosSmartContractChainEcosystem,
    #[strum(serialize = "electroneum-ecosystem")]
    ElectroneumEcosystem,
    #[strum(serialize = "elon-musk-inspired-coins")]
    ElonMuskInspiredCoins,
    #[strum(serialize = "elysium-ecosystem")]
    ElysiumEcosystem,
    #[strum(serialize = "emoji-themed")]
    EmojiThemed,
    #[strum(serialize = "endurance-ecosystem")]
    EnduranceEcosystem,
    #[strum(serialize = "energi-ecosystem")]
    EnergiEcosystem,
    #[strum(serialize = "energy")]
    Energy,
    #[strum(serialize = "entertainment")]
    Entertainment,
    #[strum(serialize = "enuls-ecosystem")]
    EnulsEcosystem,
    #[strum(serialize = "eos-ecosystem")]
    EosEcosystem,
    #[strum(serialize = "erc20i")]
    Erc20i,
    #[strum(serialize = "erc-404")]
    Erc404,
    #[strum(serialize = "ergo-ecosystem")]
    ErgoEcosystem,
    #[strum(serialize = "etf")]
    Etf,
    #[strum(serialize = "ethereum-classic-ecosystem")]
    EthereumClassicEcosystem,
    #[strum(serialize = "ethereum-ecosystem")]
    EthereumEcosystem,
    #[strum(serialize = "ethereum-pos-iou")]
    EthereumPosIou,
    #[strum(serialize = "ethereumpow-ecosystem")]
    EthereumpowEcosystem,
    #[strum(serialize = "ether-fi-ecosystem")]
    EtherFiEcosystem,
    #[strum(serialize = "etherlink-ecosystem")]
    EtherlinkEcosystem,
    #[strum(serialize = "eur-stablecoin")]
    EurStablecoin,
    #[strum(serialize = "evmos-ecosystem")]
    EvmosEcosystem,
    #[strum(serialize = "exchange-based-tokens")]
    ExchangeBasedTokens,
    #[strum(serialize = "f1-partnership")]
    F1Partnership,
    #[strum(serialize = "fan-token")]
    FanToken,
    #[strum(serialize = "fantom-ecosystem")]
    FantomEcosystem,
    #[strum(serialize = "farcaster-ecosystem")]
    FarcasterEcosystem,
    #[strum(serialize = "farming-as-a-service-faas")]
    FarmingAsAServiceFaas,
    #[strum(serialize = "farming-games")]
    FarmingGames,
    #[strum(serialize = "fiat-backed-stablecoin")]
    FiatBackedStablecoin,
    #[strum(serialize = "fighting-games")]
    FightingGames,
    #[strum(serialize = "fixed-interest")]
    FixedInterest,
    #[strum(serialize = "flare-network-ecosystem")]
    FlareNetworkEcosystem,
    #[strum(serialize = "flaunch-ecosystem")]
    FlaunchEcosystem,
    #[strum(serialize = "flooring-protocol")]
    FlooringProtocol,
    #[strum(serialize = "flow-ecosystem")]
    FlowEcosystem,
    #[strum(serialize = "flow-evm-ecosystem")]
    FlowEvmEcosystem,
    #[strum(serialize = "four-meme-ecosystem")]
    FourMemeEcosystem,
    #[strum(serialize = "fractionalized-nft")]
    FractionalizedNft,
    #[strum(serialize = "fraxtal-ecosystem")]
    FraxtalEcosystem,
    #[strum(serialize = "friend-tech")]
    FriendTech,
    #[strum(serialize = "frog-themed-coins")]
    FrogThemedCoins,
    #[strum(serialize = "ftx-holdings")]
    FtxHoldings,
    #[strum(serialize = "fuse-ecosystem")]
    FuseEcosystem,
    #[strum(serialize = "galachain-ecosystem")]
    GalachainEcosystem,
    #[strum(serialize = "galaxy-digital-portfolio")]
    GalaxyDigitalPortfolio,
    #[strum(serialize = "gambling")]
    Gambling,
    #[strum(serialize = "game-studio")]
    GameStudio,
    #[strum(serialize = "gaming-blockchains")]
    GamingBlockchains,
    #[strum(serialize = "gaming")]
    Gaming,
    #[strum(serialize = "gaming-governance-token")]
    GamingGovernanceToken,
    #[strum(serialize = "gaming-marketplace")]
    GamingMarketplace,
    #[strum(serialize = "gaming-platform")]
    GamingPlatform,
    #[strum(serialize = "gaming-utility-token")]
    GamingUtilityToken,
    #[strum(serialize = "gbp-stablecoin")]
    GbpStablecoin,
    #[strum(serialize = "genesys-network-ecosystem")]
    GenesysNetworkEcosystem,
    #[strum(serialize = "gig-economy")]
    GigEconomy,
    #[strum(serialize = "glue-ecosystem")]
    GlueEcosystem,
    #[strum(serialize = "gmci-30-index")]
    Gmci30Index,
    #[strum(serialize = "gmci-defi-index")]
    GmciDefiIndex,
    #[strum(serialize = "gmci-depin-index")]
    GmciDepinIndex,
    #[strum(serialize = "gmci-index")]
    GmciIndex,
    #[strum(serialize = "gmci-layer-1-index")]
    GmciLayer1Index,
    #[strum(serialize = "gmci-layer-2-index")]
    GmciLayer2Index,
    #[strum(serialize = "gmci-meme-index")]
    GmciMemeIndex,
    #[strum(serialize = "xdai-ecosystem")]
    XdaiEcosystem,
    #[strum(serialize = "gotchiverse")]
    Gotchiverse,
    #[strum(serialize = "graphite-network-ecosystem")]
    GraphiteNetworkEcosystem,
    #[strum(serialize = "graphlinq-ecosystem")]
    GraphlinqEcosystem,
    #[strum(serialize = "gravity-alpha-ecosystem")]
    GravityAlphaEcosystem,
    #[strum(serialize = "guild-scholarship")]
    GuildScholarship,
    #[strum(serialize = "gunz-ecosystem")]
    GunzEcosystem,
    #[strum(serialize = "ham-ecosystem")]
    HamEcosystem,
    #[strum(serialize = "haqq-network-ecosystem")]
    HaqqNetworkEcosystem,
    #[strum(serialize = "harmony-ecosystem")]
    HarmonyEcosystem,
    #[strum(serialize = "haven1-ecosystem")]
    Haven1Ecosystem,
    #[strum(serialize = "healthcare")]
    Healthcare,
    #[strum(serialize = "heco-chain-ecosystem")]
    HecoChainEcosystem,
    #[strum(serialize = "hedera-ecosystem")]
    HederaEcosystem,
    #[strum(serialize = "hela-ecosystem")]
    HelaEcosystem,
    #[strum(serialize = "hemi-ecosystem")]
    HemiEcosystem,
    #[strum(serialize = "humanode-ecosystem")]
    HumanodeEcosystem,
    #[strum(serialize = "huobi-eco-chain-ecosystem")]
    HuobiEcoChainEcosystem,
    #[strum(serialize = "hybrid-token-standards")]
    HybridTokenStandards,
    #[strum(serialize = "hydra-ecosystem")]
    HydraEcosystem,
    #[strum(serialize = "hydration-ecosystem")]
    HydrationEcosystem,
    #[strum(serialize = "hyperevm-ecosystem")]
    HyperevmEcosystem,
    #[strum(serialize = "hyperliquid-ecosystem")]
    HyperliquidEcosystem,
    #[strum(serialize = "hyperunit-ecosystem")]
    HyperunitEcosystem,
    #[strum(serialize = "hyperxpad-launchpad")]
    HyperxpadLaunchpad,
    #[strum(serialize = "icb-network-ecosystem")]
    IcbNetworkEcosystem,
    #[strum(serialize = "idr-stablecoin")]
    IdrStablecoin,
    #[strum(serialize = "immutable-ecosystem")]
    ImmutableEcosystem,
    #[strum(serialize = "impossible-launchpad")]
    ImpossibleLaunchpad,
    #[strum(serialize = "index-coin")]
    IndexCoin,
    #[strum(serialize = "defi-pulse-index-dpi")]
    DefiPulseIndexDpi,
    #[strum(serialize = "index-coop-index")]
    IndexCoopIndex,
    #[strum(serialize = "index-coop-metaverse-index")]
    IndexCoopMetaverseIndex,
    #[strum(serialize = "inevm-ecosystem")]
    InevmEcosystem,
    #[strum(serialize = "infofi")]
    Infofi,
    #[strum(serialize = "infrastructure")]
    Infrastructure,
    #[strum(serialize = "initia-ecosystem")]
    InitiaEcosystem,
    #[strum(serialize = "injective-ecosystem")]
    InjectiveEcosystem,
    #[strum(serialize = "ink-ecosystem")]
    InkEcosystem,
    #[strum(serialize = "inscriptions")]
    Inscriptions,
    #[strum(serialize = "insurance")]
    Insurance,
    #[strum(serialize = "intent")]
    Intent,
    #[strum(serialize = "internet-computer-ecosystem")]
    InternetComputerEcosystem,
    #[strum(serialize = "internet-of-things-iot")]
    InternetOfThingsIot,
    #[strum(serialize = "interoperability")]
    Interoperability,
    #[strum(serialize = "iota-ecosystem")]
    IotaEcosystem,
    #[strum(serialize = "iota-evm-ecosystem")]
    IotaEvmEcosystem,
    #[strum(serialize = "iotex-ecosystem")]
    IotexEcosystem,
    #[strum(serialize = "iou-tokens")]
    IouTokens,
    #[strum(serialize = "jack-butcher-ecosystem")]
    JackButcherEcosystem,
    #[strum(serialize = "jibchain-ecosystem")]
    JibchainEcosystem,
    #[strum(serialize = "jpy-stablecoin")]
    JpyStablecoin,
    #[strum(serialize = "juno-ecosystem")]
    JunoEcosystem,
    #[strum(serialize = "kadena-ecosystem")]
    KadenaEcosystem,
    #[strum(serialize = "klaytn-ecosystem")]
    KlaytnEcosystem,
    #[strum(serialize = "kardiachain-ecosystem")]
    KardiachainEcosystem,
    #[strum(serialize = "kaspa-ecosystem")]
    KaspaEcosystem,
    #[strum(serialize = "katana-ecosystem")]
    KatanaEcosystem,
    #[strum(serialize = "kava-ecosystem")]
    KavaEcosystem,
    #[strum(serialize = "kommunitas")]
    Kommunitas,
    #[strum(serialize = "kroma-ecosystem")]
    KromaEcosystem,
    #[strum(serialize = "krw-stablecoin")]
    KrwStablecoin,
    #[strum(serialize = "kucoin-community-chain-ecosystem")]
    KucoinCommunityChainEcosystem,
    #[strum(serialize = "kujira-ecosystem")]
    KujiraEcosystem,
    #[strum(serialize = "laika-ecosystem")]
    LaikaEcosystem,
    #[strum(serialize = "large-cap")]
    LargeCap,
    #[strum(serialize = "larissa-ecosystem")]
    LarissaEcosystem,
    #[strum(serialize = "launchpad")]
    Launchpad,
    #[strum(serialize = "layer-0-l0")]
    Layer0L0,
    #[strum(serialize = "layer-1")]
    Layer1,
    #[strum(serialize = "layer-2")]
    Layer2,
    #[strum(serialize = "layer-3-l3")]
    Layer3L3,
    #[strum(serialize = "legal")]
    Legal,
    #[strum(serialize = "lending-borrowing")]
    LendingBorrowing,
    #[strum(serialize = "lens-ecosystem")]
    LensEcosystem,
    #[strum(serialize = "leveraged-token")]
    LeveragedToken,
    #[strum(serialize = "lightlink-ecosystem")]
    LightlinkEcosystem,
    #[strum(serialize = "linea-ecosystem")]
    LineaEcosystem,
    #[strum(serialize = "liquid-restaked-eth")]
    LiquidRestakedEth,
    #[strum(serialize = "liquid-restaked-sol")]
    LiquidRestakedSol,
    #[strum(serialize = "liquid-restaking-governance-token")]
    LiquidRestakingGovernanceToken,
    #[strum(serialize = "liquid-restaking-tokens")]
    LiquidRestakingTokens,
    #[strum(serialize = "liquid-staked-apt")]
    LiquidStakedApt,
    #[strum(serialize = "liquid-staked-btc")]
    LiquidStakedBtc,
    #[strum(serialize = "liquid-staked-eth")]
    LiquidStakedEth,
    #[strum(serialize = "liquid-staked-sol")]
    LiquidStakedSol,
    #[strum(serialize = "liquid-staked-sui")]
    LiquidStakedSui,
    #[strum(serialize = "liquid-staking")]
    LiquidStaking,
    #[strum(serialize = "liquid-staking-governance-tokens")]
    LiquidStakingGovernanceTokens,
    #[strum(serialize = "liquid-staking-tokens")]
    LiquidStakingTokens,
    #[strum(serialize = "lisk-ecosystem")]
    LiskEcosystem,
    #[strum(serialize = "loopring-ecosystem")]
    LoopringEcosystem,
    #[strum(serialize = "lp-tokens")]
    LpTokens,
    #[strum(serialize = "lrtfi")]
    Lrtfi,
    #[strum(serialize = "lsdfi")]
    Lsdfi,
    #[strum(serialize = "lukso-ecosystem")]
    LuksoEcosystem,
    #[strum(serialize = "lung-ecosystem")]
    LungEcosystem,
    #[strum(serialize = "made-in-china")]
    MadeInChina,
    #[strum(serialize = "made-in-usa")]
    MadeInUsa,
    #[strum(serialize = "mainnetz-ecosystem")]
    MainnetzEcosystem,
    #[strum(serialize = "manta-network-ecosystem")]
    MantaNetworkEcosystem,
    #[strum(serialize = "mantle-ecosystem")]
    MantleEcosystem,
    #[strum(serialize = "mantra-ecosystem")]
    MantraEcosystem,
    #[strum(serialize = "manufacturing")]
    Manufacturing,
    #[strum(serialize = "marketing")]
    Marketing,
    #[strum(serialize = "market-making-solution")]
    MarketMakingSolution,
    #[strum(serialize = "mascot-themed")]
    MascotThemed,
    #[strum(serialize = "massa-ecosystem")]
    MassaEcosystem,
    #[strum(serialize = "masternodes")]
    Masternodes,
    #[strum(serialize = "maxxchain-ecosystem")]
    MaxxchainEcosystem,
    #[strum(serialize = "media")]
    Media,
    #[strum(serialize = "meme-token")]
    MemeToken,
    #[strum(serialize = "memecoin-nfts")]
    MemecoinNfts,
    #[strum(serialize = "memecore-ecosystem")]
    MemecoreEcosystem,
    #[strum(serialize = "memeland-ecosystem")]
    MemelandEcosystem,
    #[strum(serialize = "memorial-themed")]
    MemorialThemed,
    #[strum(serialize = "merlin-chain-ecosystem")]
    MerlinChainEcosystem,
    #[strum(serialize = "metagovernance")]
    Metagovernance,
    #[strum(serialize = "metaverse")]
    Metaverse,
    #[strum(serialize = "meter-ecosystem")]
    MeterEcosystem,
    #[strum(serialize = "metis-ecosystem")]
    MetisEcosystem,
    #[strum(serialize = "mev-protection")]
    MevProtection,
    #[strum(serialize = "mezo-ecosystem")]
    MezoEcosystem,
    #[strum(serialize = "mid-cap")]
    MidCap,
    #[strum(serialize = "migaloo-ecosystem")]
    MigalooEcosystem,
    #[strum(serialize = "milady-and-derivatives")]
    MiladyAndDerivatives,
    #[strum(serialize = "milkomeda-cardano-ecosystem")]
    MilkomedaCardanoEcosystem,
    #[strum(serialize = "mint-ecosystem")]
    MintEcosystem,
    #[strum(serialize = "massively-multiplayer-online-mmo")]
    MassivelyMultiplayerOnlineMmo,
    #[strum(serialize = "mobile-mining")]
    MobileMining,
    #[strum(serialize = "mode-ecosystem")]
    ModeEcosystem,
    #[strum(serialize = "modular-blockchain")]
    ModularBlockchain,
    #[strum(serialize = "moonbeam-ecosystem")]
    MoonbeamEcosystem,
    #[strum(serialize = "moonchain-ecosystem")]
    MoonchainEcosystem,
    #[strum(serialize = "moonriver-ecosystem")]
    MoonriverEcosystem,
    #[strum(serialize = "moonshot-ecosystem")]
    MoonshotEcosystem,
    #[strum(serialize = "morph-l2-ecosystem")]
    MorphL2Ecosystem,
    #[strum(serialize = "morpho-ecosystem")]
    MorphoEcosystem,
    #[strum(serialize = "movement-ecosystem")]
    MovementEcosystem,
    #[strum(serialize = "move-to-earn")]
    MoveToEarn,
    #[strum(serialize = "multicoin-capital-portfolio")]
    MulticoinCapitalPortfolio,
    #[strum(serialize = "multivac-ecosystem")]
    MultivacEcosystem,
    #[strum(serialize = "multiversx-ecosystem")]
    MultiversxEcosystem,
    #[strum(serialize = "murad-picks")]
    MuradPicks,
    #[strum(serialize = "music")]
    Music,
    #[strum(serialize = "nahmii-ecosystem")]
    NahmiiEcosystem,
    #[strum(serialize = "name-service")]
    NameService,
    #[strum(serialize = "near-protocol-ecosystem")]
    NearProtocolEcosystem,
    #[strum(serialize = "neo-ecosystem")]
    NeoEcosystem,
    #[strum(serialize = "neon-ecosystem")]
    NeonEcosystem,
    #[strum(serialize = "non-fungible-tokens-nft")]
    NonFungibleTokensNft,
    #[strum(serialize = "nft-aggregator")]
    NftAggregator,
    #[strum(serialize = "nft-amm")]
    NftAmm,
    #[strum(serialize = "nft-collections-that-received-airdrops")]
    NftCollectionsThatReceivedAirdrops,
    #[strum(serialize = "nftfi")]
    Nftfi,
    #[strum(serialize = "nft-index")]
    NftIndex,
    #[strum(serialize = "nft-launchpad")]
    NftLaunchpad,
    #[strum(serialize = "nft-lending-borrowing")]
    NftLendingBorrowing,
    #[strum(serialize = "nft-marketplace")]
    NftMarketplace,
    #[strum(serialize = "nike-ecosystem")]
    NikeEcosystem,
    #[strum(serialize = "nounsdao")]
    Nounsdao,
    #[strum(serialize = "oasis-emerald-ecosystem")]
    OasisEmeraldEcosystem,
    #[strum(serialize = "oasis-sapphire-ecosystem")]
    OasisSapphireEcosystem,
    #[strum(serialize = "oasys-ecosystem")]
    OasysEcosystem,
    #[strum(serialize = "ocm-ecosystem")]
    OcmEcosystem,
    #[strum(serialize = "oec-ecosystem")]
    OecEcosystem,
    #[strum(serialize = "ohm-fork")]
    OhmFork,
    #[strum(serialize = "okt-chain-ecosystem")]
    OktChainEcosystem,
    #[strum(serialize = "okx-ventures-portfolio")]
    OkxVenturesPortfolio,
    #[strum(serialize = "olympus-pro")]
    OlympusPro,
    #[strum(serialize = "omnia-ecosystem")]
    OmniaEcosystem,
    #[strum(serialize = "onchain-ecosystem")]
    OnchainEcosystem,
    #[strum(serialize = "on-chain-gaming")]
    OnChainGaming,
    #[strum(serialize = "ondo-tokenized-stocks")]
    OndoTokenizedStocks,
    #[strum(serialize = "opbnb-ecosystem")]
    OpbnbEcosystem,
    #[strum(serialize = "opinions-fun-ecosystem")]
    OpinionsFunEcosystem,
    #[strum(serialize = "optimism-ecosystem")]
    OptimismEcosystem,
    #[strum(serialize = "superchain-ecosystem")]
    SuperchainEcosystem,
    #[strum(serialize = "decentralized-options")]
    DecentralizedOptions,
    #[strum(serialize = "oracle")]
    Oracle,
    #[strum(serialize = "oraichain-ecosystem")]
    OraichainEcosystem,
    #[strum(serialize = "osmosis-ecosytem")]
    OsmosisEcosytem,
    #[strum(serialize = "outlier-ventures-portfolio")]
    OutlierVenturesPortfolio,
    #[strum(serialize = "paal-ai")]
    PaalAi,
    #[strum(serialize = "pantera-capital-portfolio")]
    PanteraCapitalPortfolio,
    #[strum(serialize = "paradigm-portfolio")]
    ParadigmPortfolio,
    #[strum(serialize = "parallel-ecosystem")]
    ParallelEcosystem,
    #[strum(serialize = "parallel-evm")]
    ParallelEvm,
    #[strum(serialize = "parex-network-ecosystem")]
    ParexNetworkEcosystem,
    #[strum(serialize = "parody-meme-coins")]
    ParodyMemeCoins,
    #[strum(serialize = "payment-solutions")]
    PaymentSolutions,
    #[strum(serialize = "peaq-ecosystem")]
    PeaqEcosystem,
    #[strum(serialize = "decentralized-perpetuals")]
    DecentralizedPerpetuals,
    #[strum(serialize = "pfp-avatar")]
    PfpAvatar,
    #[strum(serialize = "platon-network-ecosystem")]
    PlatonNetworkEcosystem,
    #[strum(serialize = "play-to-earn")]
    PlayToEarn,
    #[strum(serialize = "plume-network-ecosystem")]
    PlumeNetworkEcosystem,
    #[strum(serialize = "politifi")]
    Politifi,
    #[strum(serialize = "dot-ecosystem")]
    DotEcosystem,
    #[strum(serialize = "polychain-capital-portfolio")]
    PolychainCapitalPortfolio,
    #[strum(serialize = "polygon-ecosystem")]
    PolygonEcosystem,
    #[strum(serialize = "polygon-zkevm-ecosystem")]
    PolygonZkevmEcosystem,
    #[strum(serialize = "poolz-finance-launchpad")]
    PoolzFinanceLaunchpad,
    #[strum(serialize = "prediction-markets")]
    PredictionMarkets,
    #[strum(serialize = "privacy-coins")]
    PrivacyCoins,
    #[strum(serialize = "privacy-blockchain")]
    PrivacyBlockchain,
    #[strum(serialize = "proof-ecosystem")]
    ProofEcosystem,
    #[strum(serialize = "proof-of-memes-ecosystem")]
    ProofOfMemesEcosystem,
    #[strum(serialize = "proof-of-stake-pos")]
    ProofOfStakePos,
    #[strum(serialize = "proof-of-work-pow")]
    ProofOfWorkPow,
    #[strum(serialize = "provenance-ecosystem")]
    ProvenanceEcosystem,
    #[strum(serialize = "pudgy-ecosystem")]
    PudgyEcosystem,
    #[strum(serialize = "pulsechain-ecosystem")]
    PulsechainEcosystem,
    #[strum(serialize = "puma-ecosystem")]
    PumaEcosystem,
    #[strum(serialize = "pump-fun")]
    PumpFun,
    #[strum(serialize = "pump-science-ecosystem")]
    PumpScienceEcosystem,
    #[strum(serialize = "pundi-aifx-omnilayer-ecosystem")]
    PundiAifxOmnilayerEcosystem,
    #[strum(serialize = " puzzle-games")]
    PuzzleGames,
    #[strum(serialize = "qitmeer-network-ecosystem")]
    QitmeerNetworkEcosystem,
    #[strum(serialize = "ql1-ecosystem")]
    Ql1Ecosystem,
    #[strum(serialize = "q-mainnet-ecosystem")]
    QMainnetEcosystem,
    #[strum(serialize = "quai-network-ecosystem")]
    QuaiNetworkEcosystem,
    #[strum(serialize = "qubic-chain-ecosystem")]
    QubicChainEcosystem,
    #[strum(serialize = "quest-to-earn")]
    QuestToEarn,
    #[strum(serialize = "racing-games")]
    RacingGames,
    #[strum(serialize = "radix-ecosystem")]
    RadixEcosystem,
    #[strum(serialize = "rari-ecosystem")]
    RariEcosystem,
    #[strum(serialize = "reactive-network-ecosystem")]
    ReactiveNetworkEcosystem,
    #[strum(serialize = "re-al-ecosystem")]
    ReAlEcosystem,
    #[strum(serialize = "realt-tokens")]
    RealtTokens,
    #[strum(serialize = "real-world-assets-rwa")]
    RealWorldAssetsRwa,
    #[strum(serialize = "rebase-tokens")]
    RebaseTokens,
    #[strum(serialize = "recruitment-solutions")]
    RecruitmentSolutions,
    #[strum(serialize = "redbelly-network-ecosystem")]
    RedbellyNetworkEcosystem,
    #[strum(serialize = "reddit-points")]
    RedditPoints,
    #[strum(serialize = "redstone-ecosystem")]
    RedstoneEcosystem,
    #[strum(serialize = "eco-friendly")]
    EcoFriendly,
    #[strum(serialize = "restaking")]
    Restaking,
    #[strum(serialize = "retail")]
    Retail,
    #[strum(serialize = "rollup")]
    Rollup,
    #[strum(serialize = "rollups-as-a-service-raas")]
    RollupsAsAServiceRaas,
    #[strum(serialize = "rollux-ecosystem")]
    RolluxEcosystem,
    #[strum(serialize = "ronin-ecosystem")]
    RoninEcosystem,
    #[strum(serialize = "rootstock-ecosystem")]
    RootstockEcosystem,
    #[strum(serialize = "rpg")]
    Rpg,
    #[strum(serialize = "rss3-vsl-ecosystem")]
    Rss3VslEcosystem,
    #[strum(serialize = "runes")]
    Runes,
    #[strum(serialize = "rwa-protocol")]
    RwaProtocol,
    #[strum(serialize = "saakuru-ecosystem")]
    SaakuruEcosystem,
    #[strum(serialize = "saga-ecosystem")]
    SagaEcosystem,
    #[strum(serialize = "sanko-ecosystem")]
    SankoEcosystem,
    #[strum(serialize = "scroll-ecosystem")]
    ScrollEcosystem,
    #[strum(serialize = "secret-ecosystem")]
    SecretEcosystem,
    #[strum(serialize = "sei-ecosystem")]
    SeiEcosystem,
    #[strum(serialize = "seigniorage")]
    Seigniorage,
    #[strum(serialize = "sei-v2-ecosystem")]
    SeiV2Ecosystem,
    #[strum(serialize = "sequoia-capital-portfolio")]
    SequoiaCapitalPortfolio,
    #[strum(serialize = "sgd-stablcoin")]
    SgdStablcoin,
    #[strum(serialize = "shibarium")]
    Shibarium,
    #[strum(serialize = "shido-network-ecosystem")]
    ShidoNetworkEcosystem,
    #[strum(serialize = "shimmerevm-ecosystem")]
    ShimmerevmEcosystem,
    #[strum(serialize = "shooting-games")]
    ShootingGames,
    #[strum(serialize = "sidechain")]
    Sidechain,
    #[strum(serialize = "simulation-games")]
    SimulationGames,
    #[strum(serialize = "skale-ecosystem")]
    SkaleEcosystem,
    #[strum(serialize = "smartbch-ecosystem")]
    SmartbchEcosystem,
    #[strum(serialize = "smart-contract-platform")]
    SmartContractPlatform,
    #[strum(serialize = "socialfi")]
    Socialfi,
    #[strum(serialize = "software")]
    Software,
    #[strum(serialize = "solana-ecosystem")]
    SolanaEcosystem,
    #[strum(serialize = "solana-meme-coins")]
    SolanaMemeCoins,
    #[strum(serialize = "token-2022")]
    Token2022,
    #[strum(serialize = "soneium-ecosystem")]
    SoneiumEcosystem,
    #[strum(serialize = "songbird-ecosystem")]
    SongbirdEcosystem,
    #[strum(serialize = "sonic-ecosystem")]
    SonicEcosystem,
    #[strum(serialize = "sonic-svm-ecosystem")]
    SonicSvmEcosystem,
    #[strum(serialize = "sora-ecosystem")]
    SoraEcosystem,
    #[strum(serialize = "spl22")]
    Spl22,
    #[strum(serialize = "spore-fun-ecosystem")]
    SporeFunEcosystem,
    #[strum(serialize = "sports")]
    Sports,
    #[strum(serialize = "sports-games")]
    SportsGames,
    #[strum(serialize = "src-20")]
    Src20,
    #[strum(serialize = "stablecoin-protocol")]
    StablecoinProtocol,
    #[strum(serialize = "stablecoins")]
    Stablecoins,
    #[strum(serialize = "stacks-ecosystem")]
    StacksEcosystem,
    #[strum(serialize = "starknet-ecosystem")]
    StarknetEcosystem,
    #[strum(serialize = "stellar-ecosystem")]
    StellarEcosystem,
    #[strum(serialize = "step-network-ecosystem")]
    StepNetworkEcosystem,
    #[strum(serialize = "sticker-themed-coin")]
    StickerThemedCoin,
    #[strum(serialize = "stock-market-themed")]
    StockMarketThemed,
    #[strum(serialize = "storage")]
    Storage,
    #[strum(serialize = "story-ecosystem")]
    StoryEcosystem,
    #[strum(serialize = "strategy-games")]
    StrategyGames,
    #[strum(serialize = "stx-city-ecosystem")]
    StxCityEcosystem,
    #[strum(serialize = "sui-ecosystem")]
    SuiEcosystem,
    #[strum(serialize = "sui-meme")]
    SuiMeme,
    #[strum(serialize = "sun-pump-ecosystem")]
    SunPumpEcosystem,
    #[strum(serialize = "superseed-ecosystem")]
    SuperseedEcosystem,
    #[strum(serialize = "supra-ecosystem")]
    SupraEcosystem,
    #[strum(serialize = "swellchain-ecosystem")]
    SwellchainEcosystem,
    #[strum(serialize = "sx-rollup-ecosystem")]
    SxRollupEcosystem,
    #[strum(serialize = "synthetic")]
    Synthetic,
    #[strum(serialize = "synths")]
    Synths,
    #[strum(serialize = "synthetic-dollar")]
    SyntheticDollar,
    #[strum(serialize = "synthetic-issuer")]
    SyntheticIssuer,
    #[strum(serialize = "syscoin-nevm-ecosystem")]
    SyscoinNevmEcosystem,
    #[strum(serialize = "tac-ecosystem")]
    TacEcosystem,
    #[strum(serialize = "tap-to-earn")]
    TapToEarn,
    #[strum(serialize = "taraxa-ecosystem")]
    TaraxaEcosystem,
    #[strum(serialize = "telegram_apps")]
    TelegramApps,
    #[strum(serialize = "telos-ecosystem")]
    TelosEcosystem,
    #[strum(serialize = "tempcategory")]
    Tempcategory,
    #[strum(serialize = "tenet-ecosystem")]
    TenetEcosystem,
    #[strum(serialize = "terminal-of-truths")]
    TerminalOfTruths,
    #[strum(serialize = "terra-classic-ecosystem")]
    TerraClassicEcosystem,
    #[strum(serialize = "terra-ecosystem")]
    TerraEcosystem,
    #[strum(serialize = "terraport-launchpad")]
    TerraportLaunchpad,
    #[strum(serialize = "tezos-ecosystem")]
    TezosEcosystem,
    #[strum(serialize = "the-boy-s-club")]
    TheBoySClub,
    #[strum(serialize = "theta-ecosystem")]
    ThetaEcosystem,
    #[strum(serialize = "thundercore-ecosystem")]
    ThundercoreEcosystem,
    #[strum(serialize = "tiktok-meme")]
    TiktokMeme,
    #[strum(serialize = "time-fun-ecosystem")]
    TimeFunEcosystem,
    #[strum(serialize = "titanchain-ecosystem")]
    TitanchainEcosystem,
    #[strum(serialize = "tokenfi-launchpad")]
    TokenfiLaunchpad,
    #[strum(serialize = "tokenized-products")]
    TokenizedProducts,
    #[strum(serialize = "tokenized-btc")]
    TokenizedBtc,
    #[strum(serialize = "tokenized-commodities")]
    TokenizedCommodities,
    #[strum(serialize = "tokenized-exchange-traded-funds-etfs")]
    TokenizedExchangeTradedFundsEtfs,
    #[strum(serialize = "tokenized-gold")]
    TokenizedGold,
    #[strum(serialize = "real-estate")]
    RealEstate,
    #[strum(serialize = "tokenized-silver")]
    TokenizedSilver,
    #[strum(serialize = "tokenized-stock")]
    TokenizedStock,
    #[strum(serialize = "tokenized-t-bills")]
    TokenizedTBills,
    #[strum(serialize = "tokenized-treasury-bonds-t-bonds")]
    TokenizedTreasuryBondsTBonds,
    #[strum(serialize = "tokensets")]
    Tokensets,
    #[strum(serialize = "token-standards")]
    TokenStandards,
    #[strum(serialize = "ton-ecosystem")]
    TonEcosystem,
    #[strum(serialize = "ton-meme-coins")]
    TonMemeCoins,
    #[strum(serialize = "tourism")]
    Tourism,
    #[strum(serialize = "tower-defense-games")]
    TowerDefenseGames,
    #[strum(serialize = "tradable-ecosystem")]
    TradableEcosystem,
    #[strum(serialize = "trading-bots")]
    TradingBots,
    #[strum(serialize = "tron-ecosystem")]
    TronEcosystem,
    #[strum(serialize = "tron-meme")]
    TronMeme,
    #[strum(serialize = "try-stablecoins")]
    TryStablecoins,
    #[strum(serialize = "ultron-ecosystem")]
    UltronEcosystem,
    #[strum(serialize = "unichain-ecosystem")]
    UnichainEcosystem,
    #[strum(serialize = "units-network-ecosystem")]
    UnitsNetworkEcosystem,
    #[strum(serialize = "usd-stablecoin")]
    UsdStablecoin,
    #[strum(serialize = "us-treasury-backed-stablecoin")]
    UsTreasuryBackedStablecoin,
    #[strum(serialize = "uton-ecosystem")]
    UtonEcosystem,
    #[strum(serialize = "vana-ecosystem")]
    VanaEcosystem,
    #[strum(serialize = "vanar-chain-ecosystem")]
    VanarChainEcosystem,
    #[strum(serialize = "vechain-ecosystem")]
    VechainEcosystem,
    #[strum(serialize = "velas-ecosystem")]
    VelasEcosystem,
    #[strum(serialize = "venom-ecosystem")]
    VenomEcosystem,
    #[strum(serialize = "venture-capital-portfolios")]
    VentureCapitalPortfolios,
    #[strum(serialize = "viction-ecosystem")]
    VictionEcosystem,
    #[strum(serialize = "virtual-reality")]
    VirtualReality,
    #[strum(serialize = "virtuals-protocol-ecosystem")]
    VirtualsProtocolEcosystem,
    #[strum(serialize = "voi-network-ecosystem")]
    VoiNetworkEcosystem,
    #[strum(serialize = "vpn")]
    Vpn,
    #[strum(serialize = "vyvo-smart-chain-ecosystem")]
    VyvoSmartChainEcosystem,
    #[strum(serialize = "wallets")]
    Wallets,
    #[strum(serialize = "wall-street-bets-themed-coins")]
    WallStreetBetsThemedCoins,
    #[strum(serialize = "wanchain-ecosystem")]
    WanchainEcosystem,
    #[strum(serialize = "waves-ecosystem")]
    WavesEcosystem,
    #[strum(serialize = "wax-ecosystem")]
    WaxEcosystem,
    #[strum(serialize = "web-2-brands")]
    Web2Brands,
    #[strum(serialize = "wemix-ecosystem")]
    WemixEcosystem,
    #[strum(serialize = "wojak-themed")]
    WojakThemed,
    #[strum(serialize = "world-chain-ecosystem")]
    WorldChainEcosystem,
    #[strum(serialize = "world-liberty-financial-portfolio")]
    WorldLibertyFinancialPortfolio,
    #[strum(serialize = "wormhole-assets")]
    WormholeAssets,
    #[strum(serialize = "wrapped-tokens")]
    WrappedTokens,
    #[strum(serialize = "xai-ecosystem")]
    XaiEcosystem,
    #[strum(serialize = "xdc-ecosystem")]
    XdcEcosystem,
    #[strum(serialize = "x-layer-ecosystem")]
    XLayerEcosystem,
    #[strum(serialize = "xrp-ledger-ecosystem")]
    XrpLedgerEcosystem,
    #[strum(serialize = "xrpl-evm-ecosystem")]
    XrplEvmEcosystem,
    #[strum(serialize = "xstocks-ecosystem")]
    XstocksEcosystem,
    #[strum(serialize = "xt-smart-chain-ecosystem")]
    XtSmartChainEcosystem,
    #[strum(serialize = "yearn-yfi-partnerships-mergers")]
    YearnYfiPartnershipsMergers,
    #[strum(serialize = "yearn-vault-tokens")]
    YearnVaultTokens,
    #[strum(serialize = "yield-aggregator")]
    YieldAggregator,
    #[strum(serialize = "yield-bearing-tokens")]
    YieldBearingTokens,
    #[strum(serialize = "yield-bearing-stablecoins")]
    YieldBearingStablecoins,
    #[strum(serialize = "yield-farming")]
    YieldFarming,
    #[strum(serialize = "yield-optimizer")]
    YieldOptimizer,
    #[strum(serialize = "yield-tokenization-product")]
    YieldTokenizationProduct,
    #[strum(serialize = "yield-tokenization")]
    YieldTokenization,
    #[strum(serialize = "binance-labs-portfolio")]
    BinanceLabsPortfolio,
    #[strum(serialize = "zano-ecosystem")]
    ZanoEcosystem,
    #[strum(serialize = "zedxion-ecosystem")]
    ZedxionEcosystem,
    #[strum(serialize = "zenon-ecosystem")]
    ZenonEcosystem,
    #[strum(serialize = "zero-knowledge-zk")]
    ZeroKnowledgeZk,
    #[strum(serialize = "zero-network-ecosystem")]
    ZeroNetworkEcosystem,
    #[strum(serialize = "zetachain-ecosystem")]
    ZetachainEcosystem,
    #[strum(serialize = "ziliqa-evm-ecosystem")]
    ZiliqaEvmEcosystem,
    #[strum(serialize = "zilliqa-ecosystem")]
    ZilliqaEcosystem,
    #[strum(serialize = "zircuit-ecosystem")]
    ZircuitEcosystem,
    #[strum(serialize = "zklink-nova-ecosystem")]
    ZklinkNovaEcosystem,
    #[strum(serialize = "zksync-ecosystem")]
    ZksyncEcosystem,
    #[strum(serialize = "zodiac-themed")]
    ZodiacThemed,
    #[strum(serialize = "zoo-themed")]
    ZooThemed,
    #[strum(serialize = "zora-ecosystem")]
    ZoraEcosystem,
}
