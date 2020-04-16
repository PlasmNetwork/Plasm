//! A set of constant values used in substrate runtime.

/// Money matters.
pub mod currency {
    use sp_std::vec;
    #[cfg(feature = "std")]
    use hex_literal::hex;
    use sp_core::ecdsa::Public;
    use plasm_primitives::{Balance, AccountId};
    use sp_runtime::{MultiSigner, traits::IdentifyAccount};

    pub const MILLIPLM: Balance = 1_000_000_000_000;
    pub const PLM: Balance = 1_000 * MILLIPLM;

    #[cfg(feature = "std")]
    lazy_static::lazy_static! {
        /// PLASM GENESIS TOKEN HODERS
        pub static ref HOLDERS: Vec<(AccountId, Balance)> = vec![
            (MultiSigner::from(Public::from_full(&hex!["1f57f692ecc6c7031f7b89e283a14a111a273ef5ca4295ed114cef5faed3f1a66da013f76b420d7a054aec6e76ecc077a70dc9951dc41b1679e20d1d9eb665b0"][..]).unwrap()).into_account(), 216982973545339075380)
          , (MultiSigner::from(Public::from_full(&hex!["dc9381468a9712dfae81b5c88827cd824c189db9f7f49d4571cfe67b498b691204ae6d4c354a3677788828830649b682c91eeef379daec40f56a9ac7c4a669bd"][..]).unwrap()).into_account(), 245318656574379798128)
          , (MultiSigner::from(Public::from_full(&hex!["718d7dbc47423bef210b00867fad556a54f6a6ea4fa5964d08a49ba9a0759ccedb2374c5d7067af49b4aa936f1706ee69704e5315ed821fb7c9079a2ba9c23df"][..]).unwrap()).into_account(), 3964349490241977537795)
          , (MultiSigner::from(Public::from_full(&hex!["0a50f3242e28ecf0e516e8b0c2023046fbc1739f0055794b3469e5bf24996b8413036ec72faf504fc41e532f3915a5afa088f1ad2657170aeddd287163531a54"][..]).unwrap()).into_account(), 68645121610430949354)
          , (MultiSigner::from(Public::from_full(&hex!["343d24f03fe492954760e659a2db8bdfcb71318f72fe0cdf8baaf37a9699c6179a679882937b243271db39262951376ac4e2c9a00b030d7cc908417ac682444b"][..]).unwrap()).into_account(), 17846258868830223041193)
          , (MultiSigner::from(Public::from_full(&hex!["c3c5db960b08e53baa7d03cd8807046e4d520d551385721d670e38864cd0345ee6e7b2ab522bbd2dc7936af6ac196bac775d2a089b7a6f6c89a1d4444cc95674"][..]).unwrap()).into_account(), 739021074735619285802)
          , (MultiSigner::from(Public::from_full(&hex!["d3cd7302d2867af23113c06d380ccf240481a29ae77d7b42cae5255666a829a314ba690e068b089becedee93165148e1dc96031e2a45fb90b67a941562eb560e"][..]).unwrap()).into_account(), 3527157464997876307081)
          , (MultiSigner::from(Public::from_full(&hex!["2e68f7bdb547f1aab37a2e5fc81e9b9374a3828be6c45e24d0fc6a68ece72dfcafb9f5aa53becef052fe2ab69565f4184416eda56598220fc407cdb0d3a3fd58"][..]).unwrap()).into_account(), 111358131748370155555)
          , (MultiSigner::from(Public::from_full(&hex!["bffb38577d429f06620e6b380e00cd2b11af30536a94a599b948617b94513621ec17b07834efcf7c549dcab86f4e603dd7dfc1f41d372697327f8f765c11aae6"][..]).unwrap()).into_account(), 198460036779273544555)
          , (MultiSigner::from(Public::from_full(&hex!["3e909fb1f265942dbbf5e0659ee5c74820c8ecb5a55eecba5752f3cd55293ef98fa8efc553f174ddf3f7e2b5d99ab731da7cff2c1ed7d20579c59e85ab6772dc"][..]).unwrap()).into_account(), 144320138745887721599)
          , (MultiSigner::from(Public::from_full(&hex!["eba7bf9474d3fc042776d520de21788178c2d513e2079233704a29f645f7b128fbf2856cc1ffd352006895525080a56455597c4ad022880bd9a3d33fecf2f3b1"][..]).unwrap()).into_account(), 1190760220675641267332)
          , (MultiSigner::from(Public::from_full(&hex!["21ad8737c3a01fd59ae3f61d449050520be7cad34fda254c75696055acbd27b2e982a3e4dd474a32aee729256452d6814bbbf1aa9468dd9ac6a086a10ccf908e"][..]).unwrap()).into_account(), 78590174564592323643)
          , (MultiSigner::from(Public::from_full(&hex!["49161621dbfb10d39d2c68bfc876b20a25fd59e22c21e7a8ec20f651c73e3f2e6a9b766e7700cd33b308c358517a427bb9632f2fa4d999f057e602152cd54206"][..]).unwrap()).into_account(), 132306691186182363036)
          , (MultiSigner::from(Public::from_full(&hex!["29ac883d7c217923b0a71414d7cd6a8283ee2e5d87388dbbec4b264d450fabe2b4faf17c09527fb0fe53772feb34596674cc68c37a80f6db4d2fa44c97c5d115"][..]).unwrap()).into_account(), 222733904388898468759)
          , (MultiSigner::from(Public::from_full(&hex!["863c6a000311f49571545c06eb5fb7b68b2d9038daf5b38b9b8a22041adfc45f492182660648d8ff1d7e1afb8482c8521e3863e2300f91a580b77772b09ef91a"][..]).unwrap()).into_account(), 106124299556196758244)
          , (MultiSigner::from(Public::from_full(&hex!["82dfcc217121e48b86815d51bf7d02581c4b2b6c34dcf08da37ff8d9e334e490baaba2a720982fa0bed8bbb74f37e47195435b71b56bba0bc838d6187b9ac888"][..]).unwrap()).into_account(), 36788978040077890893)
          , (MultiSigner::from(Public::from_full(&hex!["52724f1effeed57157da965a63b24396b60af1621792903807e7e26702439f58cf38d69b9f697fed3d6c64ea8e28c0551eb15e51bdb9154e2cd77f3f227fa50b"][..]).unwrap()).into_account(), 74498590139659634066)
          , (MultiSigner::from(Public::from_full(&hex!["af31f6114109d0b866975b93efa71d1bb38d81dcf6ba15ca10265d1b919478487fc06fceefbe35a4d2fced8f7328839dea74e90e6ccd5fa1dc64662fdbb78b5f"][..]).unwrap()).into_account(), 39180421483268137105)
          , (MultiSigner::from(Public::from_full(&hex!["9fcf00bf3c9ba513c872429bd3b86fd2dbe6f8b77e02ebbf99bdb5ec10d9f3d4c56d1bb7f6d2d91b57913c9510ac2babb6c8ad53c29b1bbf512a257055e70eb8"][..]).unwrap()).into_account(), 27122871693167384422)
          , (MultiSigner::from(Public::from_full(&hex!["22e40099bb0bc88d3fb6705b14b803d5fb6f0c6c82ebbb4f22ec644991c2a29600af3de124a68a3f93202cf997497b268681a5feab1bc54c8ce1b97931b66345"][..]).unwrap()).into_account(), 957679933035983337780)
          , (MultiSigner::from(Public::from_full(&hex!["fa39b662ca1de2ab6bc30e7a055490c845e533e7fc406830ae5399c82a20018f1d1ce3da763308a792b6ada67dde2fd88fd6dcd71f24a8fd448874671255a833"][..]).unwrap()).into_account(), 598136499737532766229)
          , (MultiSigner::from(Public::from_full(&hex!["ac15bf47bf9043265270c6553c829b7535303c11e541c69c2ceeee7e0f2b2c55ca153f0c0a9bec1fa7e4f5c5beda14a95a121bd3a1a0b3588eea26ba533f84d4"][..]).unwrap()).into_account(), 110255575988485302530)
          , (MultiSigner::from(Public::from_full(&hex!["77f262bff093222f17021ad6f49e248dc8f03a737d0e7ae7f39ab2d9d07a031482597cf6af09e0d44ecc9471175db2e4ea2bdf8a56f8bbb9477e2fdb87ac0c82"][..]).unwrap()).into_account(), 551277879942426512653)
          , (MultiSigner::from(Public::from_full(&hex!["fcf6cac1abd4ba173dde84e3c1d7009b948f780a987a47682e9587968992afec636361caabf553aeaf6bfdd3438c82cad71924bd78724d91f298c2cbd2c27bdd"][..]).unwrap()).into_account(), 55127787994242651265)
          , (MultiSigner::from(Public::from_full(&hex!["38e0b7cbd45861d054dc3fa495f8f89ad8f897ffa765cdff26ddf890069630b6b74524e335f7392aa02086c5cb8ddea67906f39d7f1b4d4c4f20fc1003d65e2b"][..]).unwrap()).into_account(), 2877670533299466396053)
          , (MultiSigner::from(Public::from_full(&hex!["d1c51cad3d44bb8ede9dff28f326b80df16d27edbaa24aa8f17d922ce4cf4cec4b4aa39b7702edb0408c31cb66b3962b5d06da35dff5258fff7e081c95815392"][..]).unwrap()).into_account(), 178173010797392248889)
          , (MultiSigner::from(Public::from_full(&hex!["bc2dc5c8e54debc264bfb2d6d9dbba01c1d293c0975e6605c92dcde79a134dae0381ce651c1a1ba70a21508a9bde3c14d9b5906c7575e2c6b4b715273a2970cf"][..]).unwrap()).into_account(), 105845352948945890)
          , (MultiSigner::from(Public::from_full(&hex!["2385c842e30d9ea8c3711c85d5e76c9203ccd3359122f8c7ab0063ea085d8aad403fbcab43c70b5cdc8ddf22933d8c5ccfb38be3655ba0ba8ef985770d94c2dd"][..]).unwrap()).into_account(), 33341286178917955485)
          , (MultiSigner::from(Public::from_full(&hex!["08de319bb757668714fef5da8da5a5c080204390455b7923d910d4fd15af9d90300a810401e84bf1c504198a7f8ccb52bedf743103831cc97c31fcf014738bdc"][..]).unwrap()).into_account(), 2271529478745169596859)
          , (MultiSigner::from(Public::from_full(&hex!["d90c200e9ce8334206856180c11e8d50b425c9c7bc8d6ce4f92f0700c73169127ec29e0eae0c82021c6bae4a55475f1144ffffc1078b3abf25e2e5832302d9e4"][..]).unwrap()).into_account(), 33650001791685714331)
          , (MultiSigner::from(Public::from_full(&hex!["505456096e5bd4bb44f085086fa6337d54a11c88aed09ed2c44c018e6f1d5978960dbbfc996c0dfe8c8cc021107200ebf6fde369483dc1d31ce2f6af96111e65"][..]).unwrap()).into_account(), 52922676474472945)
          , (MultiSigner::from(Public::from_full(&hex!["c0ba90acf5e8cb1f3ea73edf266776e54ccdf95bf257bf371f019185d227d75fe2eb4cd9950528e28e31a41035eb9363d836e4a6c55c75324f3ba7769411ffe8"][..]).unwrap()).into_account(), 1113581317483701555560)
          , (MultiSigner::from(Public::from_full(&hex!["830df02bc5517156fc203372be4ff53117ad8defcac9468d0d94aa883dbcc5d0fa1e8cf2cea3a495853a104d9a3fbe9e45bf979a3a3b3b1770ba6107c493d653"][..]).unwrap()).into_account(), 55127787994242651265)
          , (MultiSigner::from(Public::from_full(&hex!["dbeda202f867ad2d554f88d4bca3e8358a1ab115966c9944cccc9ebf2dfe0fb8d91ce34c43327108503809966a794c4f714130db378f7db2027ba0d2a4708817"][..]).unwrap()).into_account(), 2227162634967403111121)
          , (MultiSigner::from(Public::from_full(&hex!["62a994272be9a91c938c120c02ddebc7bc74f2b2a38a97973a708242388b78ce07bba4df12e72b5877c209fff081090991936689c55e37c60c232a361b5f761f"][..]).unwrap()).into_account(), 176408921581576484)
          , (MultiSigner::from(Public::from_full(&hex!["921e9bf76b336cf585a6d861190d849b7dad1c39784a44f45c42f7ce385b14d1864ff6d55b57c3e2eb0ea0fe529e58bdc71d6b35bca7fb82c4bc4d93492cbbf7"][..]).unwrap()).into_account(), 1378062181474174201367)
          , (MultiSigner::from(Public::from_full(&hex!["c52e27276675c97d96f53650f87928d00131fca09ca79168554f4366cd1b399f9b129a64e69a302d3b4601342657518286414b66729367940825d1029449dafd"][..]).unwrap()).into_account(), 690420416839894964445)
          , (MultiSigner::from(Public::from_full(&hex!["70eb509a21bac2fb439bea4244bb1f01a74b4da4ab1c1628578fd5d9c4724a1984109e24470da94bad574818e7cd9045972365893a43f99678aa041f1f934ba4"][..]).unwrap()).into_account(), 286664497570061786580)
          , (MultiSigner::from(Public::from_full(&hex!["6d478acff9748da06e0c6a9caee369850ddd19495bd52aa81d73722369921bc0bd65b1f2fb879d640b887fc3c289bf7c43d1ee7b43fe1d0b132f8e70b0ef4dcb"][..]).unwrap()).into_account(), 1524173082464820822184)
          , (MultiSigner::from(Public::from_full(&hex!["264d2da59f9a569acd5b16b4b6ac1775a09dfd3c1acc7b2f20925b53a73aecb50d57c9800634cdf0fd9b9e5f7c026a5f3495fea85091175ff5748c741655f2c4"][..]).unwrap()).into_account(), 5345190323921767466)
          , (MultiSigner::from(Public::from_full(&hex!["cdcfe3c5be0b839f3c050ec381ac3fdd8898ceb964cf7f96ea2ed5bf3013ee44aa877f5db09de25998941ac7eccb1a4209aa1ee13f3b5d96faf5f606891a2038"][..]).unwrap()).into_account(), 278283971239177018734)
          , (MultiSigner::from(Public::from_full(&hex!["aef5ed2b29b74b622a5df66821207c7981e77dddd9fbee530a8ae732bc6ca66f8aaa89bb402ec9ad71665349605ac2e7aaa917fd13dbf6ef1cc2544a575bed49"][..]).unwrap()).into_account(), 200444637147066280000)
          , (MultiSigner::from(Public::from_full(&hex!["7a4e1723b1d86a9f9fa9463a3bf9bf74bb77fed77da8133d57d786403a29b090595d0711616ed482fc2372e4222a18d28748a60104720ce60befe03726a63b80"][..]).unwrap()).into_account(), 259613404902996729801)
          , (MultiSigner::from(Public::from_full(&hex!["2a5d75624c4cd9a56baf7bf4fc8efb3783e4048972d30404700de1fe7ebe3e618100efb8fef6fd56ad8e7cf1b09f51adaca6366141febf99222d8adec6dc25e6"][..]).unwrap()).into_account(), 5578932145017356308056)
          , (MultiSigner::from(Public::from_full(&hex!["0b360f61842987358fe28ae9746e0332ee70501d087268caf52788215ad60596821225c4a68453d23d52a5473364b017cd6e3ae9f06d647d9bc0ad109c650bbf"][..]).unwrap()).into_account(), 14994758334434001144)
          , (MultiSigner::from(Public::from_full(&hex!["4b933bce50d8fa5ff9274e5d5765cc3612983188dda322d843c7a97b30389080bfceba27e2c2649973fc2ab42e80b1d620ec316d42cf9f22236914b232520e6d"][..]).unwrap()).into_account(), 17640892158157648404)
          , (MultiSigner::from(Public::from_full(&hex!["f7b8f4942d88ffe9138931f00bf273271f7f6e01a510cf91d9e699b7e4ade16e70b08fd18425943ca613744efbbf65abee3ac75e1f2142ce5011d02be0a08de2"][..]).unwrap()).into_account(), 11025557598848530253)
          , (MultiSigner::from(Public::from_full(&hex!["d984ec212fc5d142f0821f26c418bb0f6c6cd4ab244fc04a995763f55406ed80855c048de6f6677680c94bd3fb92214f59d877349adcf7c1399691282fdfa21e"][..]).unwrap()).into_account(), 198460036779273544555)
          , (MultiSigner::from(Public::from_full(&hex!["41d45ffa33b1897f97bce7c670f030f5683ad907874b50aa5fb53ac0874192e053b12fc3e08fe3c7353ee476b51a58bb3b9e06150e4758fd1e31777bdb46e157"][..]).unwrap()).into_account(), 66153345593091181517)
          , (MultiSigner::from(Public::from_full(&hex!["fd525c3c0d9a6cd27cea0df37c27f0b4cb0e1ec394188db3cb72b2e00a2fd1b5302f0669efffc3bf6672a8bd6ccdbc58d633c4c385cdc0615251555b11fc5843"][..]).unwrap()).into_account(), 3775150921845736)
          , (MultiSigner::from(Public::from_full(&hex!["87901b216ad2690b5f07111380cb788891d5394b4c77f7a4d9df01b42ee01667b502a6045c7f9ce7739127870be90a54e330209c6750825616a61dc14e575c9b"][..]).unwrap()).into_account(), 17817301079739224888)
          , (MultiSigner::from(Public::from_full(&hex!["8d618e3d91fef8eba49a93e27214fd2af6eb1de9da46fd5d683787ca2ae4c4e312c22dfc5ef9c142d4fb68f9354432a500ed09d4ce44555b79884d0b3ac0181e"][..]).unwrap()).into_account(), 110255575988485302)
          , (MultiSigner::from(Public::from_full(&hex!["f3c2d346217373305b0e8aa028581e64905cfafed0ef05b6897d441a1db0957579ba3e60dcf2511ef47de432effbfb653f2574bd522506e7e5988c6b21ca797b"][..]).unwrap()).into_account(), 176408921581576484049)
          , (MultiSigner::from(Public::from_full(&hex!["58bc92e4c50a144f7f783b08007ae2d146fc5d305afc8ce391a8f7379769368346b78e5c15b21ceaec5da0bc4fd810b8248e888331a89bb1c96b70c341f6383b"][..]).unwrap()).into_account(), 110255575988485302530)
          , (MultiSigner::from(Public::from_full(&hex!["d7bd1da8319f105f9aed1507b125d7a1db0058feab4d9127fd910465396604c444701c8e292d100f00136f2bfa29256a3de8bbd4b7e23fc378b1becfca00527b"][..]).unwrap()).into_account(), 129219535058504774565)
          , (MultiSigner::from(Public::from_full(&hex!["de3bdb8ea24e5cff0cbc0f1e9ef8129cb8ac42fecc4ac7faf4f92a54c1379fce06518bfb15c93995a2d070157d1aef3844b3beb65e67c4ba999ddeadb81e460e"][..]).unwrap()).into_account(), 60861077945643886995)
          , (MultiSigner::from(Public::from_full(&hex!["4d65f2176255a9128e60d9a591bb80b4cf0f33001b421910740e22ee0787d520e45409698845c737999097aff7b8ff6f1244d80750e37b039496f46151364daa"][..]).unwrap()).into_account(), 16935256471831342468)
          , (MultiSigner::from(Public::from_full(&hex!["5c05b51cdb9a6a598079ed4efd054f6c5d40aa62d4f666b4ff3f7401bda644ea49444d3320ce48b5af69eadaec9b53e3aaed7a765be5c2030946c2086600c437"][..]).unwrap()).into_account(), 352817843163152968098)
          , (MultiSigner::from(Public::from_full(&hex!["276e43e6ba5d40ab26e150549b61ad787caa064477ea6ea28cca58d1c76573a809260e4e02fd0c86f3e81fb38d8d4cce8d01de4a657f3f3294dce6cdbc694b58"][..]).unwrap()).into_account(), 26543699152499871127)
          , (MultiSigner::from(Public::from_full(&hex!["24c614be394b3f776b98517497e36c79b39926e47a57901ece4bc322291378a028fdfde34927a4733d190bf23afaeda4f4f2f72e3c5af80db8533799e3f7c3f6"][..]).unwrap()).into_account(), 17640892158157648404)
          , (MultiSigner::from(Public::from_full(&hex!["f905660c6e511913cb8f3ddf35490e4095367067521331c9a8cf45e3850990a6e3d3410d993770318aea0ddc2034834ff8d4308f4b783257ad58c0184b4e1353"][..]).unwrap()).into_account(), 16423097270249630541)
          , (MultiSigner::from(Public::from_full(&hex!["fbb843968ad8995e2eb8203bf443cef70e929ec6daf64a5f6978639c5b0211bf25b23ec321607b11909168a5e2ddc18c535f74d8c899f3bfa69d9a4d5d7e8011"][..]).unwrap()).into_account(), 609128980663584750891)
          , (MultiSigner::from(Public::from_full(&hex!["5c75a9b21c960db0f1796372277c4ef22dddfd26ca3c98a7cd9bffbe8dc0b58c8cbd27abb67f5f13c93bde5f2ce7aafb9d8631dc21f9c7da8d400528e3ed3503"][..]).unwrap()).into_account(), 1362758919217678339280)
          , (MultiSigner::from(Public::from_full(&hex!["8cada4e5c0009c9b3dbf6583dc50ab5c7e207711d72d84773c57a5cef6a09c079ccec5c25f30977a67651bcb59b27494fd8f7ba8ec0e107a63a5d4504e356dcd"][..]).unwrap()).into_account(), 5345190323921767466)
          , (MultiSigner::from(Public::from_full(&hex!["d1998734ad4d17bd0487bdb48e738a61bc7dd9797599fd10f982577b1f8c2f3bf1c0371f77ef8f395939bbfc5020a275dc7cfcad9ab88388c9334955864ad3af"][..]).unwrap()).into_account(), 55127787994242651265)
          , (MultiSigner::from(Public::from_full(&hex!["365e0864f983591b37f3d60ffa662f2336f71ed893dbe2c9799f27ba65a259262d92ce97a60aa08614bca7a1010b00c252b7f6fdcbd4cf0f54e1d0525af83709"][..]).unwrap()).into_account(), 242551241617068817037)
          , (MultiSigner::from(Public::from_full(&hex!["1ccdc670d1d96c6c0c74923335b238153ff5ba473b83eb5752b6e452f12c8e12cb990b3e3dd88ba259118b9f00cd97a7fa45e11ab2739380d0c0ae5f0c8008cd"][..]).unwrap()).into_account(), 6236937422516636593)
          , (MultiSigner::from(Public::from_full(&hex!["6733facadbafa35fa40924c34701b097291581b6bc393c71f0caff5665dbbb4d26f23a1c23770091cdac1e670073b7d1b754d2d2479931cfa47c931cd77324ac"][..]).unwrap()).into_account(), 1194729421411226737)
          , (MultiSigner::from(Public::from_full(&hex!["0feb03c90ca68eea3e6e427ca0d2f4b6ad7eaf075138b08076d1277eac922356cc3da2f15c8f2dd144ea69bb92452492e0fbfd95e6e4c28a05b78d89fd2c3666"][..]).unwrap()).into_account(), 178173010797392248889)
          , (MultiSigner::from(Public::from_full(&hex!["12a1ee48a0f28a11e0094259c862557f4c09343165cbfc3661e48a1ac24a4860dcf3ebbd3b4a5e3a252a81640597d07b5991f680d52f9f48abdcabb48d800b48"][..]).unwrap()).into_account(), 389753461119295543)
          , (MultiSigner::from(Public::from_full(&hex!["17466f97954e8fc10d7dbc08e6998f9051e2d4d88b3fa3c7c8f3c1e1ebf95a12e9947bf9c92fc6c3def98dda19e5a964a4af8d4fe88437882d0cdabe141543e3"][..]).unwrap()).into_account(), 267259516196087)
          , (MultiSigner::from(Public::from_full(&hex!["5222d1d94d8ff4b29d3b705ee70d65d12203140a0594b21edcc7b162d8be1a697b62a0b7b91be9c038c42226460d4eddfe315b98cb36abfd745751290e4757ed"][..]).unwrap()).into_account(), 66285652284277363881500)
          , (MultiSigner::from(Public::from_full(&hex!["4ea5f82008cea1c32d00250454560c4acdc552f300b08615edf0235a226503a11ef3ed154f9506ace7693dd23e2038a0e37eb61c95350b1cf3238a02c4eac2d4"][..]).unwrap()).into_account(), 6615334559309118150)
          , (MultiSigner::from(Public::from_full(&hex!["b820068281ee67d480f675929e716ea4b8f66ff38151e9932e73b4534fad7a996c919bc6383adfbabff16ffc6776fa751a26a945c8ff933dc2f583f329af7b63"][..]).unwrap()).into_account(), 224921375016510017162)
          , (MultiSigner::from(Public::from_full(&hex!["d12b64d6ec0496a05fee78b43adcb313afee10006c91ea1378cead236f41bb5d99e014058f226a36ced1b6615341c4ff37f179d383669c89a7ee8335fe4757c0"][..]).unwrap()).into_account(), 222716263496740311111)
          , (MultiSigner::from(Public::from_full(&hex!["e48f01576894368871243dbe84322c24c50b430cbb111941d8871a1bd0fa9cafa62d57561da75c538b8f615046d7282a9c5c904ba50d34f4164478a464448d90"][..]).unwrap()).into_account(), 53451903239217674666)
          , (MultiSigner::from(Public::from_full(&hex!["4bf7d9856e81559ea061821d0aff7fa1322399b7ca3b900741b61404808386964ad2c934681f6eae124527220311a71cb728d1ebf4d69d2dd9c402813ec742b5"][..]).unwrap()).into_account(), 136981527608094139863)
          , (MultiSigner::from(Public::from_full(&hex!["486d44948998975a2541b1d73094486f8c280681abf22ae44d6bbd7210b3c7a30c9e79ccce310916bb2a2edf03b8eba6863ea4a6367c0ae7d5bdb0b14ceb0fc1"][..]).unwrap()).into_account(), 220511151976970605)
          , (MultiSigner::from(Public::from_full(&hex!["dde49c904689dd24a3a15334a39d9c6a8739b8d516f894aa878e858070e3b0a3b1630905331fdf2a98ce0c8a5db615dc65110c4551af395f28cd4ec85db0eb8c"][..]).unwrap()).into_account(), 2672595161960883732)
          , (MultiSigner::from(Public::from_full(&hex!["1ee819ced7355780889d58a01303dae537d54eaf8326d48811b253545ea2936f25393af452d82390a89409e3a0ca73740216883287f91c80c3fa8aa978c2c625"][..]).unwrap()).into_account(), 15434457571476080527)
          , (MultiSigner::from(Public::from_full(&hex!["a0bc8c2b3c852ffbab94f5dfce9652d0ac0a5f94eafbb8246b8aa440a3b3da24580390947a7262ea02aae7c9ecf88812b466569b6f72194686b5c23d7b7167aa"][..]).unwrap()).into_account(), 396920073558547089110)
          , (MultiSigner::from(Public::from_full(&hex!["a7d1119fbceed73e861714748ee754eee0e0bb9b800741f74ef4926b3025bd9f4e55e3f6b1d64b8e8d0ba2bbf0b7872d814446d338d520ae1ee2241a37f1527f"][..]).unwrap()).into_account(), 867579076338193148553)
          , (MultiSigner::from(Public::from_full(&hex!["8c4cea36c044f96543e0ca2c39a2c3c5a3ea1e5c5ed9db2781aee22128872da9ce60303d1e06d7cf8fc78b5ae191215907d79c1782cc261eaba86dbcfab6a0a0"][..]).unwrap()).into_account(), 1143129811848615616639)
          , (MultiSigner::from(Public::from_full(&hex!["d6d2a9f5cebd9240de2a782b7d6792563dcbc2a389b9c087935b782101a4acb52a916d881ad90264e5335bdb9061b3a1574d7018574fa9e31356439758fbf2f7"][..]).unwrap()).into_account(), 7349636695392430266)
          , (MultiSigner::from(Public::from_full(&hex!["1e6f5a54bc6072d8e2e831de6060f187c56726ee1c875c143faa2b9c0e31c52aba7a32fcdc54c21dc21ee4bc614e586d2dfefb1599a397caae0c352d8e7561db"][..]).unwrap()).into_account(), 7772797596036236857)
          , (MultiSigner::from(Public::from_full(&hex!["b43b47c1f54b21f9164391dace3537e79b2b02fee5eeb9a5abafeab3b19687830dbfcfdbd54d4af90c8a83f0b198b2625b0cff927beb79f32558d3c35b3b776f"][..]).unwrap()).into_account(), 748326645349047445336)
          , (MultiSigner::from(Public::from_full(&hex!["2fcfb687d49b1f18b370910e498c328afc41c388e1264553b87ea672e7b1d820e3b7a6cc05aadf74b8e8c3c6313d5fc7c56aae83ba343ab1c077ca5e80316675"][..]).unwrap()).into_account(), 12249394492320717110)
          , (MultiSigner::from(Public::from_full(&hex!["b0b5bcba85e7c45c0ee754099a8a2dfa652dfcd8027d8aa9cb1390b083d9b297e0ebe59743c20c477e3a1236d61f03fdb287d9a0a263538816c1f149ea8a4517"][..]).unwrap()).into_account(), 1380840833679789928894)
          , (MultiSigner::from(Public::from_full(&hex!["fd47a2943b04b63c125da143d59c2aac147e4a562cf76cb7bfceab518bc5755ce5c34513c135f67507009f61cb5d0203aa400ae4ff658821bccf065d199f4e31"][..]).unwrap()).into_account(), 213617091321562596064)
          , (MultiSigner::from(Public::from_full(&hex!["be63168b01b5c46f4ce491982b10c4b2c1f89090159affbca030f0db04132cf77343db86b07928c059bfd5dad68393925a86091d06c4a31d438a14ccada80fba"][..]).unwrap()).into_account(), 238152044135128253466)
          , (MultiSigner::from(Public::from_full(&hex!["8779ab3027cdc80e64d095eb3930d3a4680a2611dfed49e2be60a1e63afde506f0c232c70982c2a2e18c162219d00742d9b7b2f718f8b70717d8b43d22f9ed64"][..]).unwrap()).into_account(), 17146947177729234249)
          , (MultiSigner::from(Public::from_full(&hex!["cffaceaf106884e6a6eeea3faef84cc0a3c8df290043bbe92434f301d441ad9c83a9eb38874b2595d7ee8df6c04f79149ee5cf7dc0d80d66ba073e0e9a185ead"][..]).unwrap()).into_account(), 99009507237659801672)
          , (MultiSigner::from(Public::from_full(&hex!["ac130448fb3d8f26cf85126156fd3f45fe70a0b08c40fce15501e3aaf33a9c76626a3ccbb7e8bc7326081e270ddbad4668f7975d549691ad7aac4efa5e1bc66b"][..]).unwrap()).into_account(), 793840147117094178221)
          , (MultiSigner::from(Public::from_full(&hex!["d7097ab74b8c5d6f4142f15705b7f02ef1d125977fb77daf1fbb0910cc928b109c9f4feb58566727451f1c11e211ea801213e621abdb5762fb3bff75247e9b7d"][..]).unwrap()).into_account(), 1234862451071035388344)
          , (MultiSigner::from(Public::from_full(&hex!["445a0d02f437785eb07e5a93d774b289b2187b039571ec2c5f4532423148fd6bbed83c920daca8ee1f02a61c2e692e54d0e1ebc182a7e0701ca689ac2fbecf5e"][..]).unwrap()).into_account(), 2019868921439932124125)
          , (MultiSigner::from(Public::from_full(&hex!["228ef2eb65ed72469c43afa5cb3b13068eba8e59cfe271a6752e5bd210ded4bc5b6e20c1a57eac0515431c841a22a6b87b6cff2e8f0b0fc46a8ba7b4786d6a91"][..]).unwrap()).into_account(), 110255575988485302530)
          , (MultiSigner::from(Public::from_full(&hex!["debe8d830b4243a2eca0845875d3ae15cabc89d352de6ff9796fd4faeb850d0c8f996d55caa1fe6c2a9306b10e6703a907ddc5efd6224b9b03be238dc8919c14"][..]).unwrap()).into_account(), 793840147117094178)
          , (MultiSigner::from(Public::from_full(&hex!["a1f02fbbc222f5b5a8c33482558c41394027616e93d64744792c24ede0e6f4891a2af231c611eb74f824200dc4ed7af04fbd874601a015af900af65821b19e23"][..]).unwrap()).into_account(), 8820446079078824202)
          , (MultiSigner::from(Public::from_full(&hex!["cb9142fc1eac76a2c8a689a89504bfea405d372153deec84a25fc7ae3c3d874e88de3b26ad5ca5b97f7bb90307fa28fec29fe20ea8c8acf854129c4c49d283b6"][..]).unwrap()).into_account(), 198460036779273544555)
          , (MultiSigner::from(Public::from_full(&hex!["51893bd0730dcf8a790f925a89f5b3c24cda4edf58e9c2f0f393f8b15d879e7d49bd32c72577661dfb14fef4e863937b68a42f303291adc18f4b2bc9c8d1d729"][..]).unwrap()).into_account(), 59979033337736004576)
          , (MultiSigner::from(Public::from_full(&hex!["3eb18dc27a57af5d740827c3d28c9e772f4884bed769e2f4e7bd80494a64e44b95acec9777f4a629d5fbf897cce147da1e77e6cc971148e6892b61c783422ce2"][..]).unwrap()).into_account(), 3307667279654559075)
          , (MultiSigner::from(Public::from_full(&hex!["9b4da7d381a4e6ea212e18de5148ce73a5085a664d10c0e76a34644ea3769b11a2c52c99ce79cc074c9d9f06e3a7a7b943dcf53426edba4c24001c07a67fd0a6"][..]).unwrap()).into_account(), 19977648835657605906)
          , (MultiSigner::from(Public::from_full(&hex!["d9a7acd385b8956e6b636f57c13876f3fcba8415881b217403d1e1eca3523723c5c797a4c6a2f6f3ac5e15ad049270d6775dec6c6ea03de17ca638bf1c0ffdec"][..]).unwrap()).into_account(), 869695983397172066362)
          , (MultiSigner::from(Public::from_full(&hex!["a467184d3f3990e0470151af105ac60ddd6a22e0af2d8842079a315bbc2014303a448853841ab5fc0cedf5350bbe9fe5d0139ce4dc59bc5b717cd62fc652d035"][..]).unwrap()).into_account(), 501111592867665700001)
          , (MultiSigner::from(Public::from_full(&hex!["998a8ab4565a8333ea9e9df87d3cf59e140bc2cbec13ffbd44d6c8bf040432a11778cef03c5c77f362220cf6c732f490c2b7e267c32532eb24b43f7f0f92b144"][..]).unwrap()).into_account(), 44543252699348062222)
          , (MultiSigner::from(Public::from_full(&hex!["f8658b11694695932fff5612e37289be9c4283b4681818d03f2ed0d2403e2175b694ad9d4b988a8b1c73eaf488d3c471ad32a5629e2c5e12214ee3f49a49eb6e"][..]).unwrap()).into_account(), 31753605884683767128)
          , (MultiSigner::from(Public::from_full(&hex!["b7bdfd607fdcf532c310bacbfbf12dc04614b02d7cc4576ed9f7c44f919d992fdcec96c1f959b60c49e48e146f60e8bf801ef3ca3bcc38835bbd638a456a373c"][..]).unwrap()).into_account(), 38589451595969855885)
          , (MultiSigner::from(Public::from_full(&hex!["09f404788b911399152c537509c9ddc6bbec9830a1295a248334fd8741c19de93b6eeb81cd881d8780ff5a6e916eb8474bdd425144edb08196f8b5313755c914"][..]).unwrap()).into_account(), 190521635308102602773)
          , (MultiSigner::from(Public::from_full(&hex!["b9fef1f3dbb9dc0fa747b3da16f83cc5886740675956178e3f9b26163fc2f987517be5d4b088a426239ad35b31e1430b37cce847b191e57c909f3cd01ed95507"][..]).unwrap()).into_account(), 97995155938565736888)
          , (MultiSigner::from(Public::from_full(&hex!["2d04adf5773587ad36bbb1a1fb6da95cb70adb3ca6694e840a175927e80deca5e2b6e10ae314a78ee22a1518c745f2e479836c048fa46eaca31f859a778996c6"][..]).unwrap()).into_account(), 21751450244651044439)
          , (MultiSigner::from(Public::from_full(&hex!["728d7bdfc76de3aef3600fff46f33ff5f25adee4f1019ff87a15920929946231229671b7679ab3017913ff065a8ff1d77da34b3f44b66ccb77dcdd0f158893a6"][..]).unwrap()).into_account(), 17640892158157648404923)
          , (MultiSigner::from(Public::from_full(&hex!["f9e9177cd5da4a6a3068b00bfef78222ccdf5a2c0b7e0aff833b886eba169b94aa24a7c0e572e0f6de225f2a8dd85711b217bda41b9d1c09622a9f36efa0ef80"][..]).unwrap()).into_account(), 551277879942426512)
          , (MultiSigner::from(Public::from_full(&hex!["1f368b0e0a36fdcddb7dac3188882d977adb25b19df54977996317003b6a3809b62fb71fa2baf87615df93d6192e0d15d451171d9abf12979635667261e41642"][..]).unwrap()).into_account(), 43529893700437893806)
          , (MultiSigner::from(Public::from_full(&hex!["143351eec3ae622cdaac44de4c52f299ca2fa4e04ec9c143681e78d1aee16e2c4d08eba4e822f32ef0a0450826b7c379adc6b8ce80ffea7253101cf94dcc7ce0"][..]).unwrap()).into_account(), 793840147117094178221)
          , (MultiSigner::from(Public::from_full(&hex!["5dd14be882d78f3247094d056c5beb70959bd4bbf671f1adb36830cd510fbb49c9e8abca39106711c616a34f942de039b4ad6a96a8bf49e20c9a7cec2ca790ce"][..]).unwrap()).into_account(), 50276542650749297954)
          , (MultiSigner::from(Public::from_full(&hex!["b08809047c68793f866ccc80ad9c2198884ae453bc0416ffdfdc4e2813fb3b973cc223391a29aafcb0bdeb63c29e9c594c69369a14db8ddfa1de14b8d44fd0db"][..]).unwrap()).into_account(), 2458832092256712382736)
          , (MultiSigner::from(Public::from_full(&hex!["8c61474f42bbc0139c0354ae62a9cb4058791fc544eb0b4c4cdb5ca6f79180cdd3138e181d5a52ee32c8d201dece9f0244f63bc6809fd7fd5e3ae32f13a37ff7"][..]).unwrap()).into_account(), 39692007355854708911)
          , (MultiSigner::from(Public::from_full(&hex!["b8f1eab4fae9776c080183ce93d985891de3a1a485ce0f242c308393b7fa4eda928a18e49bb4e5a196804a2af6fb0a8ffece5d8844fa9adbf810e9255d610ba9"][..]).unwrap()).into_account(), 1058453529489458904295)
          , (MultiSigner::from(Public::from_full(&hex!["4de306f614512f449ca698e82d6bf24718d367dc131af924df69b1f03d3988ead7d4e5ec8044956cc0d725384058d0cd5ebd8936316c9b42c8f1c9226ac158d7"][..]).unwrap()).into_account(), 111358131748370155555)
          , (MultiSigner::from(Public::from_full(&hex!["be9d6862fc8667061d3e2254e8d35ce430d2727fe9d8ffc3506501fb8d437e666df457ddbe939516fcf5636c7cbeff4e01341e509d96b379ba867dd0a3170c04"][..]).unwrap()).into_account(), 29989516668868002288)
          , (MultiSigner::from(Public::from_full(&hex!["f1020ffb6093e1bd52a81f949d2eb87d5e75c47c0d0b304089242442be5b82cb291faab6e6647d15e55b36408b6892bae2a67802e41f319483770b23e53a6dca"][..]).unwrap()).into_account(), 8017785485882651199)
          , (MultiSigner::from(Public::from_full(&hex!["4449c706a0f8b2933aad819f5c671d77a01346a7c898620a1efdd58f0ba6dd5ec5c4d84809beb756ef1313b2931d3c20bb34fcfdeb6614751249b3e4211ff8ac"][..]).unwrap()).into_account(), 22271626349674031111)
          , (MultiSigner::from(Public::from_full(&hex!["b2d26c4842822ca1ab4bfbef93ad662ea732636cfe63cda67d9b609dda989d8fb9f99fff274952acf45b8bd5e8a9bcd3f1568d7cf61320a06d1ece693c943215"][..]).unwrap()).into_account(), 52922676474472945214)
          , (MultiSigner::from(Public::from_full(&hex!["cead3d2f15adb6250154540c7d9be8abb1ac30998fe5fbf3c0cf2f9dcc2bccf01e468a919d26565e282651739eb0d7466d080cdf5328d57acda82eae37fba143"][..]).unwrap()).into_account(), 264613382372364726)
          , (MultiSigner::from(Public::from_full(&hex!["639a49163ab1b65aca3780b40eaa1afaa452db5b5e195bfe250d39a188bf365e974077fbc4d34febe6c37d7ad2f00dca82162b6855568dc93b5386c5542157c9"][..]).unwrap()).into_account(), 53451903239217674666)
          , (MultiSigner::from(Public::from_full(&hex!["b57a5acdd49f2d393b5f5af3e02edf58a0a6e3a372fcefd9bc10fce574186ecc3adc1c30ddbe7af234f749d26993514fd81382c4060910f1b371a02c7f8e70ee"][..]).unwrap()).into_account(), 1323066911861823630)
          , (MultiSigner::from(Public::from_full(&hex!["14cd1e94f0635f3349c4db318635f39096e9fa14589acc1cf51f69a36bcf8cd9084600b4313df733f6e0fe9bbafe7bc0d3c64217adcce233b792ba302a464145"][..]).unwrap()).into_account(), 3836894044399288528)
          , (MultiSigner::from(Public::from_full(&hex!["b320d2adbf122f61a9e420b5790be062f1d5f99d51046ca4045dea62bebb04e9778aa8f54d64381bdf2b84005d012cc02d3692d4d60e835d51429df643f56351"][..]).unwrap()).into_account(), 219038137481764441416)
          , (MultiSigner::from(Public::from_full(&hex!["aa79b269efe1befb34a4e0e36f38b51a42b7505468a4f77acb498651f41163b3d8a6609e83457670b705fd08efec66edd2043f6f7bf39c8fdac22c01112a7df7"][..]).unwrap()).into_account(), 2205111519769706050615)
          , (MultiSigner::from(Public::from_full(&hex!["d011454c38d1525c376281945f25f8c2760d883890d90b50132872bb439221c69f9cf578bd4d665f384bd201d5cb6fba0d66f34637bd961062ab8a9931e53274"][..]).unwrap()).into_account(), 1764089215815764840492)
          , (MultiSigner::from(Public::from_full(&hex!["7068121f22850b1830fc5a7a9ab4695774943d16e7a5479204b8d1d65ea882ae2e4ac9313d3478b3335b9f2004eef462947fe13d502132b9c7a62f65457eed66"][..]).unwrap()).into_account(), 2756389399712132563268)
          , (MultiSigner::from(Public::from_full(&hex!["3b3a14ef500e405fceeb403046b20a5048de0f01c01862ded0361629ff68c9754f8541fc1cf8ed3a34e8311a40a73acb001461315f8506e262c242425898ca99"][..]).unwrap()).into_account(), 11135701816705267185451)
          , (MultiSigner::from(Public::from_full(&hex!["5fdd254d97c96cb523efb4d5599c66c9eee695e307d7de208e385bf3b30696d3126969005d3302ebe1d8f4eb4d605e8494c2aae644d78b92baab50d6bc91559a"][..]).unwrap()).into_account(), 606405667936669163919)
          , (MultiSigner::from(Public::from_full(&hex!["fd4df0288da7a57ab8f1203d719e33cee9682c0cc81ce6b5e840bfaae6c44b0117bc6d6d5e840184cf39d0830f40c5fcd416501b45a26c8b84fbb337dc8f584e"][..]).unwrap()).into_account(), 517649929265938495381)
          , (MultiSigner::from(Public::from_full(&hex!["71a14a9eb26ab24624c96cc79b5ae68abfa433f5685e44099bbcb3fa4d01848fb33f14266f86ab59a0e379a38d434b5f751f5fe295099cc228bbae45bb46bf5c"][..]).unwrap()).into_account(), 230985431695876708801)
          , (MultiSigner::from(Public::from_full(&hex!["addcca4da1a7e1b5824a56a95a9e4de5f714b34178bda70df07e548c67d47970fa022e47521f185fe8238042c17c8972fc3fc3c12a0fa5e7b1905372587b8b25"][..]).unwrap()).into_account(), 226613798107933266556)
          , (MultiSigner::from(Public::from_full(&hex!["c9eea120a23151edec0e65cb298bd2acb49ac020bcd7edc20f213fdf3880059bfd9980f07c6ac17965d1efa014aab10564de61039bc00364d2bba813a0e40d19"][..]).unwrap()).into_account(), 49710270012472437439)
          , (MultiSigner::from(Public::from_full(&hex!["bf6074bd07d72e58897f7f531ed5775020c0abc41f896872fd0fb1e0815638f7bab5f833927c28cf9b386b10077f1e82753f79c4947125bcd46482f0bef8646e"][..]).unwrap()).into_account(), 17505498310843788452)
          , (MultiSigner::from(Public::from_full(&hex!["cffa02b62b71b51292a03176d4ff1ccd3c687be4191802884a5ac7beb6ff2e69128489936e87fceeb4ece49276d81c44b13a544beb2f914d3946a03d077e7549"][..]).unwrap()).into_account(), 12160307986922020986)
          , (MultiSigner::from(Public::from_full(&hex!["d4594048edfd5df08d5e0103a78cb11e816c81d6a2d1a756a74c38c058abb5fca4c2e92960e8a8540e812fcb31792713d44dab75fcd64cb851747c21677d6782"][..]).unwrap()).into_account(), 10155861615451358185)
          , (MultiSigner::from(Public::from_full(&hex!["a96d5b471516c8256ae9e600076307846bcce715687e960d1732fea08bb7f98ecd86a959366d2d55a4e94e5d5629d3fdf7ba35d674a1ae93401da68927893e1c"][..]).unwrap()).into_account(), 4410223039539412101230)
          , (MultiSigner::from(Public::from_full(&hex!["87255f427e251a755bac17fc6b019727c70413e58d04f8df5b429c3c473484ea8d9145c92bd568ccd58f3bdd98516bd729f69bfb75e3176cdcc27bf17bab7ddb"][..]).unwrap()).into_account(), 2205111519769706050615)
          , (MultiSigner::from(Public::from_full(&hex!["82c3d74af768236822428aab6710a56d7dfc1985736cba0db7dba81c25b0ccf7bfbc966a01146e455afe5ac0329936050ddfc4e9851626f85eff208914d10e74"][..]).unwrap()).into_account(), 4454325269934806222)
          , (MultiSigner::from(Public::from_full(&hex!["4641e08600cf35a9f2652c3934e88c5fc5878f6370c54962882bc71566c473d264d68d1d176b539c84b8039acc355d265cbbf5963d6786a13549a79446a9a145"][..]).unwrap()).into_account(), 30066695572059941999)
          , (MultiSigner::from(Public::from_full(&hex!["bda9a3192ddf4fb0927bd6b38064be63d73e03e8acfb31c4e2d91bf7c13233e8e739f9a329e125aa994da3a88fb87b130aff1fe7acb40a2fe3ed1e45e99290c8"][..]).unwrap()).into_account(), 3969200735585470891107)
          , (MultiSigner::from(Public::from_full(&hex!["a69b8ae35e4488bc65b47b84c09a29aa503f91b5570b3b2f8797b135701b2d2ccaf16bd4f19413029d74938fdae939dcc1883696b817af4784ffb98360e38838"][..]).unwrap()).into_account(), 209485594378122074808)
          , (MultiSigner::from(Public::from_full(&hex!["cfd3b07ee3c514591f1447f168c70a4594da9c8c95bd47e47ee53a899e5e3bde025c76f999613c689cf22d3218a852f3776a27e3f6280c70be0cf274ea2e0f9e"][..]).unwrap()).into_account(), 1984600367792735445553)
          , (MultiSigner::from(Public::from_full(&hex!["d54b36f81adc287aded76f9e1de96662c6a41d2015ff44c3a4a44894a5a729533400966e63e165102a6bcab6400a7e594b925ba839fcea972aea5cf9d52959f7"][..]).unwrap()).into_account(), 18708166133726186133)
          , (MultiSigner::from(Public::from_full(&hex!["abde74c6c3157010cd6dbd666a7d7289d2065c561448ff32f84a88655a950301059ff5083f03ec5b3d3544b6b2209ae775617d94b1ef9f7a01ec673b39e66694"][..]).unwrap()).into_account(), 531872898568453099407)
          , (MultiSigner::from(Public::from_full(&hex!["cb7a367212275d374767c0aac76685b29487eefc72dd3108f388bddcacdfed49341d6c8c9c1e9c0db7063a23b3ebe825f219cc253422bfafd8ce5c94beafd03a"][..]).unwrap()).into_account(), 8820446079078824202)
          , (MultiSigner::from(Public::from_full(&hex!["671ceb98f280c8bb44f4db2025a019d9f49f798aaf3b977ba1ded9817b42c6e905b8b9de2fd062afd49a4d22fb3c88abb27920f67a625261a6a122920e648083"][..]).unwrap()).into_account(), 17640892158157648404)
          , (MultiSigner::from(Public::from_full(&hex!["615088ce33a89f5ecc30fa1326c4147ac080eec9d6b5de73541e555f8bab7157cda68f35cd40abfbe9468177b6625942a004d8d0e0f1e9b21a53cc6a075c94e7"][..]).unwrap()).into_account(), 200444637147066280000)
          , (MultiSigner::from(Public::from_full(&hex!["5489f4dbf7533559412e875d5c798b6ef2fdd81515b1a578967ef326537963891f14c5d39e11676c1655b7321c35ca5f3bfc46f1fc80a4bbcfe37fa2d925a71a"][..]).unwrap()).into_account(), 29933092275300135049946)
          , (MultiSigner::from(Public::from_full(&hex!["1a2413f58d5f7cbcfe5d14fec5867805fbd60e187c0e94f01e647b9afab155c4fb79c3c196714b9201f3bb687d3f71e6a0aa8bf0ee837c19055e26e807f28907"][..]).unwrap()).into_account(), 1521526948641097174924)
          , (MultiSigner::from(Public::from_full(&hex!["95b9b0bec7baca9c5ecca4525f4ec7197d59dc3be041e0de7a9b64215f3f1130c7344bb1a615fe3862e7f8b255f1d03c39261b31179781c5503a8e7115ad99d0"][..]).unwrap()).into_account(), 170675631630175248317)
          , (MultiSigner::from(Public::from_full(&hex!["7aa882b5357fa3de550e686146eac929c9e6060e26ded933d04f6d65fbdc4563d9008ba8e60af14ddce2e19bb452a5b1f2425bd99e10d4a5d3ca96ca026a7e17"][..]).unwrap()).into_account(), 53451903239217674666)
          , (MultiSigner::from(Public::from_full(&hex!["2686f389a46232d2351450b69b279be365f17ff98783fa45dd3a8d1388abd6ad52ff9e469001be3eabbd2a262cd8ae5680c9724d4d9228ba0d3de9b58f942212"][..]).unwrap()).into_account(), 11135813174837015555)
          , (MultiSigner::from(Public::from_full(&hex!["d25d6853558580fdac1eb68b881eeae7ddbe11b4d0da08fd587b396448ac6d5d66a34adde20f05e40ce5d11b88cde37afec1a2d497ffcd7b34a46fb47c4f3aa3"][..]).unwrap()).into_account(), 308715612767758847086)
          , (MultiSigner::from(Public::from_full(&hex!["2478c4eeaa1d2086da60647172c164fb289345a8d24fd6adc916f899c22233c3c1dec7bfb7306ddc6ae8c6bbdf20f05449cbe0676eafc64d7a56159aac8a1dc2"][..]).unwrap()).into_account(), 13362975809804418666729)
          , (MultiSigner::from(Public::from_full(&hex!["86d9556aa54ef4d1e7fedfa2aedb43ebb87208676ab9967a69787a297c8acb7afae2b0d360bc175f6ce92b1a45faadeeddf41162a432da0caca291c4abb6efe0"][..]).unwrap()).into_account(), 1664969453002116553516)
          , (MultiSigner::from(Public::from_full(&hex!["5451c5a057b9cc3e74cf122502bb3a1a6ca4d28427ea144535ba62c9c530eaf66fa8b94edb3f2b04fdf917f0f392764388820ce24921428361cb02674b596ca3"][..]).unwrap()).into_account(), 136716914225721775137)
          , (MultiSigner::from(Public::from_full(&hex!["e2312ffc8dfd924c5f3cb24f94999b1b7b193f9997c07c7199ca40c382d2a831016a2e7014922c85e3314793ce70c45478dd23de38ebf573ab7a048ae674ba30"][..]).unwrap()).into_account(), 2205111519769706050)
          , (MultiSigner::from(Public::from_full(&hex!["e3e45cc3cb82be1cc72655b770d4b6e177bc9573cf4250250db89e6a09251304abee34c9b4c5b6bc663180288ffdf05592369446772a783e3715a04db35855de"][..]).unwrap()).into_account(), 2205111519769706050)
          , (MultiSigner::from(Public::from_full(&hex!["f2cd9a812385dafb7ebad5bedec407794caaca16c8c743411eaccf1f382dfbbacee9aff8399589fd47b7c11ea93f50fa6325001251e284be504339bed903385d"][..]).unwrap()).into_account(), 55679065874185077777)
          , (MultiSigner::from(Public::from_full(&hex!["a9a4ae45a251d665074fd36689ab138a1acc3b39cca2f8ee97cbfb85b3835b5a71746132c5049babb0a1755a8633071d0de0f423578b7d247d924a866de4ed9b"][..]).unwrap()).into_account(), 66814879049022093333)
          , (MultiSigner::from(Public::from_full(&hex!["83495e0cf03ce217483d8708265ee07e46468f8b93d5efcb23fe1ab29fd6b3f237d178c8dd917fd466268bbdb91995ee6fc1a6fa55b86ad8d5af2bc9b767ecc1"][..]).unwrap()).into_account(), 27563893997121325632)
          , (MultiSigner::from(Public::from_full(&hex!["85cc3f6877018119fd282e6821b826f5000aca5846e96106ad6d84f50ac4cbb2a9d94e103afe3f51837981894c60a045660e05ef207732eb2c4effe5a91aafb8"][..]).unwrap()).into_account(), 343997397084074143895)
          , (MultiSigner::from(Public::from_full(&hex!["4bd97f9b86495ee3955f723a68cbf12399ecc9ff734994ab2dacbf371b88a2aa7f1fc5f9696111ca2bf8682a337fcbd138190e510e25d9455dbbe8e3f03d0aff"][..]).unwrap()).into_account(), 52922676474472945214)
          , (MultiSigner::from(Public::from_full(&hex!["bdcdaf37e0b22524179b612d63d3f80c454a1af8fa5a2e3d4ab536a4398f398bd8bafd72680c62dea43eb4789a5e06ee13fd69f578f94e59456442d3305bd513"][..]).unwrap()).into_account(), 1113581317483701555560)
          , (MultiSigner::from(Public::from_full(&hex!["ff3ce6e4e8e748932af864e21f2ee9881bd29d784abe9405e5cb76420a5fe45cf9b58deda618d7c39d8fd7b5607376b1847127ad158536765066a71dcac30c16"][..]).unwrap()).into_account(), 5679264719166877933)
          , (MultiSigner::from(Public::from_full(&hex!["a6139503cb5d7ec4d27e77165081ce0796c823dff2dae54f8379aae3ae9946003df762249c5e3c0b0330e96a810b092687d44d11f6e4ca295cf3d0719cbc8280"][..]).unwrap()).into_account(), 69916368401578184893)
          , (MultiSigner::from(Public::from_full(&hex!["aca9d2b2d3a76da3fb8c79761eef03f747653a9833692d5b543a8c6ece439cde3f1529812acb4424bbdad19df11a1490ae4a70edf6cdc4105333d961f7cb2a88"][..]).unwrap()).into_account(), 26725951619608837333)
          , (MultiSigner::from(Public::from_full(&hex!["405da7c6a3d07bd28adebf858b6cc4d0a3602ce6fa65997b33b8d7115dadd4633ebbd21dc3c402ffd37701290894a7f8b09e49ac9b1858fcfccb1d300014818b"][..]).unwrap()).into_account(), 160355709717653024000)
          , (MultiSigner::from(Public::from_full(&hex!["a1d039eca03a6cb28b3d5afe4dc71295303173f2cc3b84d8376d514dee8ff0db4c07f728112622c32d4079430708742fd1673e7d105d4efce084362c51d0bc19"][..]).unwrap()).into_account(), 6911172320801421915597)
          , (MultiSigner::from(Public::from_full(&hex!["e30fea26bbb749e04666c300600b62f421eae220a9e3bc4e3cea05bc7dd3623cadb4703451deb3e75dd15a1630ea5c3d4e13cedcd5301bde965a89d811f4a2f7"][..]).unwrap()).into_account(), 3572280662026923801997)
          , (MultiSigner::from(Public::from_full(&hex!["da0d43d509a4a9bb07f3d5c5e8f066426d7dee3d099262d21e498315081d68eb96986aff99bf0c3bfa4efbf1d8d7153c1bf6fb65d6b5509031a03305a7ab009b"][..]).unwrap()).into_account(), 1102555759884853025307)
          , (MultiSigner::from(Public::from_full(&hex!["7432d9f6a105f5e030297cb0129301664198cd0194730c6467edf5fc1be242e53e714220118e622fb6fda6789e371d263fe6e29c15a2e95b3314a2c3bed58ae3"][..]).unwrap()).into_account(), 26725951619608837333)
          , (MultiSigner::from(Public::from_full(&hex!["5e83bb5426e9be93afbd34a17d24fe9d281bf48114c6677b077e62a9d9644c99f9848c0772efba9331fe8e34834a1fc7fc6ed7e4bdb4d36d11929f055d7ab62d"][..]).unwrap()).into_account(), 119076022067564126733)
          , (MultiSigner::from(Public::from_full(&hex!["a652ec310f095cfc5817936b7fb74b53cdc9679d6b406b94c89e28b50011524ad411255f86079da2150750053ebdb31d3ce81868b782471ce0e3a36d48304d22"][..]).unwrap()).into_account(), 1781730107973922488)
          , (MultiSigner::from(Public::from_full(&hex!["d0ab19648fd12b6cfcdf3d8db4ece449ea2842c0e1f51c4a746ac6413ef17f614e0d222e490430fd36ee80959b0a4ebd275139d06d4c3eebd53ea6c8c6fcb7fc"][..]).unwrap()).into_account(), 16703719762255523332)
          , (MultiSigner::from(Public::from_full(&hex!["2426f018a31620494e1d80c7284347a0a6e520b6a418ffb6645887deec7f2f76e1474df540d35123a67cb043fb5e8bd512bf5e982c8bfcdaedd9c39fd6906fa6"][..]).unwrap()).into_account(), 2646133823723647260738)
          , (MultiSigner::from(Public::from_full(&hex!["58b421edd7dc67c98e3d281b3790aaa484ce6f668926ad75f5e5dc3b445ccdad68bd5b4c7096caf3d6af12a4e110d3adf42498f65f7a379f503f1ede975ff069"][..]).unwrap()).into_account(), 26725951619608837333)
          , (MultiSigner::from(Public::from_full(&hex!["bfcd44068378c4e3f3477f83495210a1e01789f4176414c1269ccfa4279102f7e1a3cc1962ffc09b1983912eca4e8a88e3f0deb99bd05b60f210329ae25df26c"][..]).unwrap()).into_account(), 110255575988485302)
          , (MultiSigner::from(Public::from_full(&hex!["78f53c9e8e9ac0165e65b06ff3d9b34c119d22385c1144361bec65b3e8989827ebf51f15f3c94b76e622d5d621a8713b45f7d18345eede3055efdc5531320cb7"][..]).unwrap()).into_account(), 882044607907882420246)
          , (MultiSigner::from(Public::from_full(&hex!["a855e0aa3a0519fb5eee26cd92b91324e3d56122c2731a296f3a2d40da299abafa537794814fad53ed117edebacbef7689fa1ed556cb57f9f10faa6a1049e663"][..]).unwrap()).into_account(), 134070780401998127876)
          , (MultiSigner::from(Public::from_full(&hex!["9670719fd875541ddcdb3b885d39c3130cdcac9d629214fd4c494eec6bb9d0173bb00ad5c5ceecc5eddcda5a21b3c47dc65a509d0e5bcf5f3575d067471fa608"][..]).unwrap()).into_account(), 400889274294132560001)
          , (MultiSigner::from(Public::from_full(&hex!["d3df3f54357e69c695b40c4dd1b9093de68978fc229c3c9636f9e3d5d9802dbe6f6d5c89bbb8d2814a1b116ad430c29f38dad876483078fbb415257d3d63f50c"][..]).unwrap()).into_account(), 1670371976225552333340)
          , (MultiSigner::from(Public::from_full(&hex!["60145a28fd3b91e646e18eaa84b055082f6eb0f9ef8c97833e612cc1fbe70b7c1cf50c0e14b014a75c82b6ca40f2ebc836d5af2d420993a74dc3a1597f213673"][..]).unwrap()).into_account(), 26461338237236472606)
          , (MultiSigner::from(Public::from_full(&hex!["c4c300f70f96bc1730cfbdc4ebed5aa293875ad130118e97e13e82a2d6d455cd3d2375cdd869f1d5f67d6d59077cfc5ce161226ba3c226a050d6ebd8a68fe185"][..]).unwrap()).into_account(), 11025557598848530253)
          , (MultiSigner::from(Public::from_full(&hex!["b24b47763fe9d829b21bad28591084592e023d8e2c8ce817d296ed9d743c5a3f6c82b628a35ddd0927c496cfa8a148b442a34d0db1e1216d8a88184000386d2e"][..]).unwrap()).into_account(), 2756389399712132563269)
          , (MultiSigner::from(Public::from_full(&hex!["bf86a538288fb6c004d030b7da23338fa5dd7a7e5956a77a759d5178f23b0a132ccb7fecefce5d8940ca33a7b80efe4039e291f5c280bea1eb13951d4088a0ba"][..]).unwrap()).into_account(), 5512778799424265126)
          , (MultiSigner::from(Public::from_full(&hex!["d7b0683c5cb41a5a9914311fee89edb810a13aea1b9a1fcfdb4e19b75be3cb1587c62b37458043325c98c6c816e9c562e91cdf2292a02c8764a133e734a3fe9a"][..]).unwrap()).into_account(), 37486895836085002860)
          , (MultiSigner::from(Public::from_full(&hex!["5dee8ed95e0d9bed5d85b73db0271a0e67a034473e533a882cff19f0b0751f85ccbbe195da23e36f2eaaa64d05be4ea9f0010a4f1b7185d51dc76c2f2212dc48"][..]).unwrap()).into_account(), 7216006937294386080033)
          , (MultiSigner::from(Public::from_full(&hex!["2f71f9f4ed9ff5fc8f9c922a1f234e851abad3ee64a4ff0d31582d62e61f7b9a9e1f2789b1dfd0dac595141c780c9b89d048eff7677f279de8ac8c3412d929e7"][..]).unwrap()).into_account(), 44102230395394121012)
          , (MultiSigner::from(Public::from_full(&hex!["c968d8b73b2cca3eac1ec51d76512168b483147578c4fd35932a122be4adaee4fcbde1f5d0a110825f5b70f3ab29ac2333db8e9379f2a37254661169f90ba1cd"][..]).unwrap()).into_account(), 941847232324036848337)
          , (MultiSigner::from(Public::from_full(&hex!["575e2cd7d3bb94a57d42767f01370bfdb126e21a0c4b3ebaa8bdfb0bb4604723426bddf9d0b2891dae12d5dfb2ffa41b8523ddd284eb277ab35563b47a519aa2"][..]).unwrap()).into_account(), 211690705897891780858)
          , (MultiSigner::from(Public::from_full(&hex!["803db65299c2f28d1d7d3d07f96bc668c3676a322138aab33d1d9c423a923c0fb4dc90853f0b727a31b2180ce4f25de92bebd40fc8a925b0ef9124e8f84dadba"][..]).unwrap()).into_account(), 595380110337820633666)
          , (MultiSigner::from(Public::from_full(&hex!["6501958429e3990482396ec75e739af365f34d99ff9cb76edc75d4322793fdb0b98d8f51540809c59ccc3b35faa4b49625b44ef01b3f1d0831e2e3169db10448"][..]).unwrap()).into_account(), 100553085301498595908)
          , (MultiSigner::from(Public::from_full(&hex!["e3bfddeb6e217e6c10d6310dbffb1f46c8d01cd39fecef8f6e438e998f7ebb7300b77508263784c44f3d4cd0d152aa63071641b6199f29305b7dc2202dc4a809"][..]).unwrap()).into_account(), 26461338237236472)
          , (MultiSigner::from(Public::from_full(&hex!["84ec6c3e5f5e27d09c07b1c10747d2589db22048afd2a0b8644d8bfa2c972f43fbe490bc83864cbbebf71ea9e75dc5894784d2725b6fa756e93c758f42b89d95"][..]).unwrap()).into_account(), 33076672796545590759)
          , (MultiSigner::from(Public::from_full(&hex!["eeb2974a25e268b3c9299abaded54b8fe22852177fb0fa20a89fa99c66fcd33f414f861de31b126c0970c85fe59b1000a3754fbac3edc1d2d016d757aef5a0be"][..]).unwrap()).into_account(), 745327693682160645107)
          , (MultiSigner::from(Public::from_full(&hex!["dc26c421c46b48c743b8009496ea4c46c67b5260bbbf4f8e5d9b0c9bbe5ccbd13dacc2c4d822470797a174d479c43297bc85c24caf12a4d669655a5ba8d6cf9f"][..]).unwrap()).into_account(), 6879947941681482877)
          , (MultiSigner::from(Public::from_full(&hex!["eb44ddc77960f7e04e6879ef166c7329d9a4bf7c3fa5abf4f041ce40a2b301f8a824e5fec6f10dfff68bc79f2e4b7994bb866e76b0d8766973b6aa21ec511ffd"][..]).unwrap()).into_account(), 4643634093907035486688)
          , (MultiSigner::from(Public::from_full(&hex!["5f1ebfed2c9c777529b45767a34f8b28ad8116940decf25740d5564f052f25e6d2ad2e37ef4b4c879ce705ee8025fabc8af673bb097de363e715ee54db567e39"][..]).unwrap()).into_account(), 111358131748370155555)
          , (MultiSigner::from(Public::from_full(&hex!["48a5c0d40977d72f42705dbe6c1ca07607663f6dbd1eccbd1d9c302002c296188fc3aedbde8292f18947829f26d34a7bc54fe31db73abe0cc0787b1e311ffec5"][..]).unwrap()).into_account(), 7056356863263059361)
          , (MultiSigner::from(Public::from_full(&hex!["8db20ce6f045a39b23effedf39810da67ad2ab38a65d6f43f09b106f6b5fc7af4b5e54913434e03d8de8084901913b82a09fa2e1458ea95085c25f1e3295cf30"][..]).unwrap()).into_account(), 793840147117094178)
          , (MultiSigner::from(Public::from_full(&hex!["f0d899c7b0c71cba61dd66d7339b6cb16d150ba727d02ba8f1d6adbb797f581c768aa7cf36002ac9062e9cc14965c489fe1859465a8bf8ead6f047e61a54c103"][..]).unwrap()).into_account(), 1290096084418226985498)
          , (MultiSigner::from(Public::from_full(&hex!["aac9efae605590ccd264382be9eeeab36b43d4711ef4599ba199553cc618d0ee77c5e7b46aa0a90b41748fdfe579e6f2168ee645a1f09f8f154ae3660b5a5b26"][..]).unwrap()).into_account(), 22271626349674031111)
          , (MultiSigner::from(Public::from_full(&hex!["01b2f8cf89551b0f3cd6a633369428e93bfe4719682c0c05e4f51edd4bde7960ea19cb622999aba9c96e8fb42fc469166b2947f457ef1b946baacdecbd548302"][..]).unwrap()).into_account(), 5512778799424265126)
          , (MultiSigner::from(Public::from_full(&hex!["dc83707ffb60cd7ca862b79ad34c8e3ccac3195bfbd8b1d4f5eb1f4e5ea4d567982ce2f7c0c65f469907030ab4fda0eefdf8442231a49e4245160b074f969207"][..]).unwrap()).into_account(), 1102555759884853025307)
          , (MultiSigner::from(Public::from_full(&hex!["e6418ee459cc4025a5fd4f03ce1ebdcefac7fefb36a111defb1044f4fca3dfdeb1442237c217399be4cd609d04df22e3d9a5d6844c882bff36ab7011cb21856f"][..]).unwrap()).into_account(), 2672595161960883732)
          , (MultiSigner::from(Public::from_full(&hex!["8d97b7b48e76c25fad60eda44d199cc73c8e056e4cbc17746d2544bb55ef3cc64e398ede5d2e109e1171ad7f77101b5ae283e5e3f7509de6098672beb5363363"][..]).unwrap()).into_account(), 2205111519769706050615)
          , (MultiSigner::from(Public::from_full(&hex!["7f02951e35f1c469f5f04f883ac7519598dbfcbc5c86641774b57b104be745c3b26b0b619b77520f0904fa40aae6d70c72b4d591e7f637b4a83fbadd40cfa370"][..]).unwrap()).into_account(), 11135813174837015555)
          , (MultiSigner::from(Public::from_full(&hex!["d6be5c677d9e75fddd80b130dc2236bafe0d7ec5cf052df1cfda9eb9b08c956a811f04110608f3c38214374a1706a512e16123d23f4a1e5760a1ee70e7fa98df"][..]).unwrap()).into_account(), 1764089215815764840)
          , (MultiSigner::from(Public::from_full(&hex!["69319da5dfdea2cbfe757611597eaf65c5b53341a22b42d0bec108a4c57a61b2ce5a97192d7136eaf79713ee694dc9101319a456bcf8f8b0dd1f0b33898f2cf1"][..]).unwrap()).into_account(), 31312583580729825918)
          , (MultiSigner::from(Public::from_full(&hex!["dc5e389d8532076a9a2039f4f9063241c976afee7f21c87d8ef7c7fdff89ad54a9040b9808fb9fcde180d2563835646c807c0807603b43f68080816225856ff2"][..]).unwrap()).into_account(), 3563460215947844977793)
          , (MultiSigner::from(Public::from_full(&hex!["733fd5ae8801239b92c2c24e88543f716e1ecd68fcd5343d2ce208cec9d1d462c0db2d123b580e0e054ba178e7d8e852a5bbb0cb5fa51834991c3e972af5197e"][..]).unwrap()).into_account(), 1764089215815764840)
          , (MultiSigner::from(Public::from_full(&hex!["cf104605e035586b2346dc4ea6b4e270ff241bf2d590cc58f7434f419e0526795bb386a877164ee9c4931f3181cbe9e83509a9081caeb10c2d7c3fffc086d5d1"][..]).unwrap()).into_account(), 8908650539869612444486)
          , (MultiSigner::from(Public::from_full(&hex!["20572fbd8acd8ff57d9f57805291b6c96a81d957cf082f2b7fcfadcb2b76688243d6476b90ae817caeb8d091c0b5503d454b195eb5a6451eae1e01affc34722d"][..]).unwrap()).into_account(), 356346021594784497778)
          , (MultiSigner::from(Public::from_full(&hex!["c03b87c65dc3c789560ebc63c6d9c4fae75775bfd2488d58d9fe19fee759e80d21d42f520d2ab4018408804dd606f0e829ae66c26210a9caa4bf03d6fbd46812"][..]).unwrap()).into_account(), 264613382372364726073)
          , (MultiSigner::from(Public::from_full(&hex!["683a0c3db178b92866780c8f27fceba50a8c0162fefc25c8bb4db3995eb4906623556c8ee403f6710a7b881ed285e3892c6d9532d096803b7bad1be9efd21972"][..]).unwrap()).into_account(), 303116834619063563422)
          , (MultiSigner::from(Public::from_full(&hex!["24a0d6457fe036e25278161eb8cc4d3f93834d6f0abe60f980bb9670ac0edf34afb0ea0b75218ab685e67755f105cbeb7e0ba63bb0a32ec53514c9e8ee9cff60"][..]).unwrap()).into_account(), 556790658741850777779)
          , (MultiSigner::from(Public::from_full(&hex!["1568335d7e4934dc75cd4e430c1092d4e5cc66057b5d0d83f41ae17b568f41f59f4e86546ec8fa4414671f224530cb580a0b6cc9a963e93f06b765ff2b7ceafd"][..]).unwrap()).into_account(), 306030854210654913653)
          , (MultiSigner::from(Public::from_full(&hex!["19923b4c9d654331b9101da4db00caf8a0628577eb161e441b2cf5af7404537b12b3857d175ab87d85c6220aeb87eafb258eb7a1a20a4c1d9c1f16ea1f28ef55"][..]).unwrap()).into_account(), 176408921581576484049)
          , (MultiSigner::from(Public::from_full(&hex!["32a4d72f635a36e0b4089b7820bef0833ca97e7dde113fed4eeed88aafa60e457f48dc30416aa25e088d77aa17daa167bfa6437143f3984d8d45678f6a4a9027"][..]).unwrap()).into_account(), 549293279574633777206)
          , (MultiSigner::from(Public::from_full(&hex!["a27c1e09c563b1221636c7f69690a6e4d41e9c79d38518d00d5f6d3fb5d7a35407caff68e13fcd845646dc848e0649417b89acf1af435bd18f1ab2fcf20e2e61"][..]).unwrap()).into_account(), 17639128068941832639)
          , (MultiSigner::from(Public::from_full(&hex!["d60adc4d0b8ccc948a3ff12eca7b05733073f7ef08c2d5166e2b52167acc437e251e3027ecedfd05f347e18efcd631c4a51cd9c3767c6dd80743f0cf748f5fe4"][..]).unwrap()).into_account(), 909573220120687430579)
          , (MultiSigner::from(Public::from_full(&hex!["2c2e80eff66d5a2f896d45513b7edd9aeb44a6d37e63ece4d3a1d364a5abe6f4134c73e0c8400d5417a7c4d3c3a82ac0081dbb50cc8b299cb849e9c4c1b8b7eb"][..]).unwrap()).into_account(), 591979828374335746933)
          , (MultiSigner::from(Public::from_full(&hex!["5f952fe18fcca5e4f923f464745e54daf9c610a81c4a95bc39a311814bad9bf0195021b112882e3676e890b15024cef81c218284fa4db85fda497aa9d3182a99"][..]).unwrap()).into_account(), 297548928031645055643)
          , (MultiSigner::from(Public::from_full(&hex!["fc617b898fb984cf1dc945ba44cd1200671ac0cfd867418ef994357bfe7214acc8a33fd7e1198d56afa65266536c30d6add2e345eb080e6c063936577aec62fd"][..]).unwrap()).into_account(), 612469724616035855558)
          , (MultiSigner::from(Public::from_full(&hex!["bad569927e636b531055067878e5fd9b2a1714fef235a6eab3d16b7eb01daa28c7f415f6ce7c1eb7eae9e35fe72d6fe19abe274a441dfe4dfd1b5c4aa1611660"][..]).unwrap()).into_account(), 8908650539869612444)
          , (MultiSigner::from(Public::from_full(&hex!["e974d44a67df89e96db03aa9129ee403fe56eb484ee87bcff824bf05686b3dd8e10ff80ae8b9476dc89fda88e77f6ae71d6161579684864354f0bcd513c0e011"][..]).unwrap()).into_account(), 17817301079739224888)
          , (MultiSigner::from(Public::from_full(&hex!["5b476aac1cc17b941c21a495ddbe71ff55d5472334afa282bc2f8ef3dfb9a9115ce2e1b3443092bbe34adc6a782491f84a8f32ce0bccc7824b11a771d7fdcacc"][..]).unwrap()).into_account(), 1268380146171534920314)
          , (MultiSigner::from(Public::from_full(&hex!["4e175a18345b68f67b5d7ea622809de0c1e5f5bb1ba046c5538930139e16e2de639525cd588022b835d085c90cae1e2be5a65bd19a22ff0b5043b261e3566f3b"][..]).unwrap()).into_account(), 4454325269934806222243)
          , (MultiSigner::from(Public::from_full(&hex!["479f0ba1d95830270f13c6e8200b38c0af4c9941ad5f3972057e01b0e84196f9ae7a3a88b6b9312cbac79a088f0dd5fccb0298cbcc4b110664b1d3f916f8a1b1"][..]).unwrap()).into_account(), 365410573519101828439)
          , (MultiSigner::from(Public::from_full(&hex!["558dbbd54e45c48cc036456e6dd2405a5a02a7c4f341b031b597cf1f4b6f50579e0c10f001226a284cf54142ff6874cdc88cc0458daa8db1c4fb0606f3355a7e"][..]).unwrap()).into_account(), 178173010797392248889)
          , (MultiSigner::from(Public::from_full(&hex!["99f18d1fdd52d74226beb168b238b881837dae2d183816b9623cc726cf41875616854fc608ee9155dfe08d2d75939d52a59d24cf23962ccb4471f75bad027996"][..]).unwrap()).into_account(), 174644832365760719208)
          , (MultiSigner::from(Public::from_full(&hex!["b03c9cf76b7d92cdd7ed633f2c7e3017427dcfa61a9f4368ecc74893d88e7566fb582890dc29e3d9e2ca2cdc7195dcb7fa6aaaff0b7971982e3b41fa0098a7a4"][..]).unwrap()).into_account(), 629173444378291378890)
          , (MultiSigner::from(Public::from_full(&hex!["8eab9556992d9d5b94ba74babf07f2ffa86d73e0b7c727fe8f1e99bc4c589e37b124baf03f78d78b8b7e0b0c1b0d5155b96fd5465ee45a9fbed5c9daaa80b9c3"][..]).unwrap()).into_account(), 783961247508525895114)
          , (MultiSigner::from(Public::from_full(&hex!["cc93420076ae005968ddf46012a7c772cb051f1f8d1697a1553faaf8e53cc245402ff9f65d7d443ba822e2e2d99d27f425c1d4dbce98b459ecda34eaa3c8ae18"][..]).unwrap()).into_account(), 1675884755024976598467)
          , (MultiSigner::from(Public::from_full(&hex!["879594d97fc4a75c40470df2c8bcf051d1e2b169e93ac3e6d34e5553c2e8c0ef432288a72174ee30be70c2e4793872b601a053db43d7d3d73e2652e3a6526085"][..]).unwrap()).into_account(), 1675884755024976598467)
          , (MultiSigner::from(Public::from_full(&hex!["b166545a4188e428d2161b135a7e868a5e51bc8d190d81c709fa48436afc9e4a997782383c2636404d3ab1aa4a2538c0dd325369033b876684efb4a7d80d9d5e"][..]).unwrap()).into_account(), 368694646105494851662)
          , (MultiSigner::from(Public::from_full(&hex!["b643abc51e44f0665bb5baf3ab29db2afac87b00be6478088971fbf6de8948ab6038f8f9e1187d20fbfdf32edc823f1333875ced3301442867f11c507e718081"][..]).unwrap()).into_account(), 2132730939244785219209)
          , (MultiSigner::from(Public::from_full(&hex!["03d6e6bb0cab8e2a4790a40d262bd387f70d043fea09ac75c3323e243ff848356a187afb9dc3fe2a12c8beecc24332247f459bf5cf6679979677b7ee3250ff33"][..]).unwrap()).into_account(), 3034233451203115525646)
          , (MultiSigner::from(Public::from_full(&hex!["ee5a834fda15c0dbdf68e81a1e8a6a125ba0a00ad7c0b9683fe12e8ed2454d3a2169e2972070213b0d0d127ee69c7238dd74012a10581f3a8ddabf79f8fcdb9f"][..]).unwrap()).into_account(), 529226764744729452147)
          , (MultiSigner::from(Public::from_full(&hex!["33e877e456014d8e4c862f82c4f02a9d622f096b95e1e9b7208019f90d38be05665cc50201e30d5d9ec23750e2c87a732b82d6b5f55cbd1a5b5c2e6c3e8305a7"][..]).unwrap()).into_account(), 5167017313124375217800)
          , (MultiSigner::from(Public::from_full(&hex!["232f79816af7abd3f0587e9ae2e8fe7a0ed2e1f552af80c8aab6a93b24e8155c71f9fe31c5558e77686a02798817187a3b53d1cf752e5971d17c98528521e6f8"][..]).unwrap()).into_account(), 534519032392176746668)
          , (MultiSigner::from(Public::from_full(&hex!["c3e5f176b07437365e642f27fe133cf72e575c16497ee2a8ba376ed702dd1ad2cfe0fea88d87147a0a91eb2a7d47dd4938b9b36ade14c7d9c0e5c962d79d61dc"][..]).unwrap()).into_account(), 8195958496680043448927)
          , (MultiSigner::from(Public::from_full(&hex!["fc551818250bcb1d275b938c430290523b37aa3f1075ac8024cde8b32ff4e3d4c01a1aaff17efdcc912f4a1b85cd3351de8b901271a626fa031b8b22bc6f9e2a"][..]).unwrap()).into_account(), 1051220763704614268448)
          , (MultiSigner::from(Public::from_full(&hex!["57f02a71796a7dabcc00804b90cd587dd7e04687bd1d07b46cf18f3ae394d7219dc5f1f2120fba0066db64839449e845dca22508b3328fbacb2fae5e4b30ffad"][..]).unwrap()).into_account(), 1781730107973922488896)
          , (MultiSigner::from(Public::from_full(&hex!["2f05926c3f0eede9c10e7d185383900df3b678b1c3af45c56b80511f88ba57974eb02911b908ab6ae275d26ca042e32e5cccd8d31a01457e98dbbf01a7931aff"][..]).unwrap()).into_account(), 178173010797392248889)
          , (MultiSigner::from(Public::from_full(&hex!["ee82b3da2b3c4997fae4ce8dc95da120d09cb6cbbca0483b8885a730485ff33b020145432b8876bca2493a7d54ce3c0cda3c73a90b6c11a476286569aeac8981"][..]).unwrap()).into_account(), 535768385641867309458)
          , (MultiSigner::from(Public::from_full(&hex!["aef34c7b6674d8cec3a0228e98feafebf17da836d52d89720846e3ab07a2441245d3854f994a52034b7b309ab13e12d167640683e3d40494b56c8569046b7c65"][..]).unwrap()).into_account(), 2458787549004013034676)
          , (MultiSigner::from(Public::from_full(&hex!["82a292d912ae706590f1475a5e2c5af2ec4347972d9a4db7c89f69c1e09ef4fdf8c1abc803a68352cfa2731de20cc9917bd085176209e8e7b2d9fe0b1f6e230b"][..]).unwrap()).into_account(), 220511151976970605061)
          , (MultiSigner::from(Public::from_full(&hex!["45676662fe9eb04e97ed74b1a681161543e8519f50b5633e96133e278436b5fc481d86a177a3d9a60fb07657934bcc9921e897464e4269b77f5b5f94c990feb6"][..]).unwrap()).into_account(), 3563460215947844977793)
          , (MultiSigner::from(Public::from_full(&hex!["21174af29e8129cb23639ee671adc3be95391b9596a96911e9e0bb1c8e3d359f844e0baa205ea2e3ec157cdba2d9abc0bd75e0d1fcd9a8b0a5f93299fe2940e2"][..]).unwrap()).into_account(), 2421212448707137243575)
          , (MultiSigner::from(Public::from_full(&hex!["10373ab587328461ee25317e925999078e5708328b5211f4e42246ceec311a84f63eb25509b27664add0e1e65c988d77883470a2c339b46b66572cd4b783d106"][..]).unwrap()).into_account(), 7126920431895689954)
          , (MultiSigner::from(Public::from_full(&hex!["b99599b85ae0274d5a570d301705e7797212aabd50297aa53e95358201af4a4500a7e228eb27f8afbbc1adb1dff8dd1a5f68dd1636e05babcf3994efc4848635"][..]).unwrap()).into_account(), 672382604608178768953)
          , (MultiSigner::from(Public::from_full(&hex!["2445781e6671a62a034c046cb520771bc7e302d9b446a2eaf6c7217467b0a393827fbc9eb7d6d3baf4263cbfd627da3680e450949b8b573049b8c47baf8481d7"][..]).unwrap()).into_account(), 26461338237236472607386)
          , (MultiSigner::from(Public::from_full(&hex!["239e8f64e31160d21e7da7efcaa4920c4066896399ae0770f36a7fe9871cd5bfe549f59489c69067954190bd3f5b292f013fe86570e11086b759d423945427c7"][..]).unwrap()).into_account(), 60135395590491354663083)
          , (MultiSigner::from(Public::from_full(&hex!["5ae3c95414793303b9a46eccd424b789cd4d873e5ebee2d2a8bcfb2ca3fe6b9323025774b7cf15a4af777e953a16fc117187ef70580be9dfaf9d0235e8eb51f1"][..]).unwrap()).into_account(), 3969200735585470891107)
          , (MultiSigner::from(Public::from_full(&hex!["9b9be3dcefb7184344b31386d6164717f49ed3971280532edcde1fd9ad5386fffb560042f5df49863481a5f4ba9ef2fb3ca3fbb86b5ae3a130fd2ad4e90ab715"][..]).unwrap()).into_account(), 176408921581576484049)
        ]; 
    }
}

/// Time.
pub mod time {
    use plasm_primitives::{BlockNumber, Moment};

    /// Since BABE is probabilistic this is the average expected block time that
    /// we are targetting. Blocks will be produced at a minimum duration defined
    /// by `SLOT_DURATION`, but some slots will not be allocated to any
    /// authority and hence no block will be produced. We expect to have this
    /// block time on average following the defined slot duration and the value
    /// of `c` configured for BABE (where `1 - c` represents the probability of
    /// a slot being empty).
    /// This value is only used indirectly to define the unit constants below
    /// that are expressed in blocks. The rest of the code should use
    /// `SLOT_DURATION` instead (like the timestamp module for calculating the
    /// minimum period).
    ///
    /// If using BABE with secondary slots (default) then all of the slots will
    /// always be assigned, in which case `MILLISECS_PER_BLOCK` and
    /// `SLOT_DURATION` should have the same value.
    ///
    /// <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
    pub const MILLISECS_PER_BLOCK: Moment = 10000;
    pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;

    pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

    // 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
    pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

    pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 10 * MINUTES;
    pub const EPOCH_DURATION_IN_SLOTS: u64 = {
        const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;

        (EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
    };

    // These time units are defined in number of blocks.
    pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
    pub const HOURS: BlockNumber = MINUTES * 60;
    pub const DAYS: BlockNumber = HOURS * 24;
}

// CRITICAL NOTE: The system module maintains two constants: a _maximum_ block weight and a
// _ratio_ of it yielding the portion which is accessible to normal transactions (reserving the rest
// for operational ones). `TARGET_BLOCK_FULLNESS` is entirely independent and the system module is
// not aware of if, nor should it care about it. This constant simply denotes on which ratio of the
// _maximum_ block weight we tweak the fees. It does NOT care about the type of the dispatch.
//
// For the system to be configured in a sane way, `TARGET_BLOCK_FULLNESS` should always be less than
// the ratio that `system` module uses to find normal transaction quota.
/// Fee-related.
pub mod fee {
    pub use sp_runtime::Perbill;

    /// The block saturation level. Fees will be updates based on this value.
    pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);
}
