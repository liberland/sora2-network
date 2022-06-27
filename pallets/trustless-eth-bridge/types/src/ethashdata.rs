use hex_literal::hex;

pub const DAGS_START_EPOCH: u64 = 0;
pub const ETCHASH_DAGS_START_EPOCH: u64 = 0;

pub const DAGS_MERKLE_ROOTS: [[u8; 16]; 512] = [
    hex!("55b891e842e58f58956a847cbbf67821"),
    hex!("fba03a3d1902b9256ebe9177d03242fe"),
    hex!("2b186dc65b93be71780e5194fd44fc70"),
    hex!("94c0532d49523cd9309057a847ef0dbd"),
    hex!("f61d6da773315bdd4c79418186ebaa4a"),
    hex!("28e89dd2e1e5e09ee3e4cf412af58a0e"),
    hex!("54a0171c74e7336634f5b6b61f2b302c"),
    hex!("3be685b693d9ddfc342406fcc8d98512"),
    hex!("1887acc39d0818a7c6d47e33904a150a"),
    hex!("e1434e68f6a9f30252e2f31be8db9658"),
    hex!("a5e981ffaa1f770de8a1d21550f49755"),
    hex!("f4a55238db60864330a300e1d05dba16"),
    hex!("f4b2032ab23f95f9c9516db6d43372ce"),
    hex!("5fa11b8f22bd56e5bbb4cb0f843b6730"),
    hex!("ad4e75d7abf04b5798d8d0c832bf6833"),
    hex!("7df3208dec48fb446e0f89da95843d8a"),
    hex!("250e4cae8e10486589190b68608af301"),
    hex!("a55b182e12b1433a4935514bb729d2b2"),
    hex!("99456d6b4f8886afbbafa6a758830a92"),
    hex!("cfd122fe8a0b3c8984e1a603e97bae53"),
    hex!("0d05ebdd6eae46efa4b0c7694e6db158"),
    hex!("7e59bb58278cbd8f9470fe8636c4edee"),
    hex!("c48e2800c2442220eb1d0a9d9d08b437"),
    hex!("185f8beff965e31b7859b9b63fc79f97"),
    hex!("6e6c22abdb238266d3fa0f2902f85d7c"),
    hex!("7345950e2b649e10596ae6be11782110"),
    hex!("0cc51bae63bfb29add017e4a0f89f97a"),
    hex!("0a5a13ee1aea57228395fc64b8a1852e"),
    hex!("ecb847d99f761b457747886f4e0c81d7"),
    hex!("9eaf4241ffab9b2d693b96420dbd0356"),
    hex!("93f46416f3ef2d5ea57fe1a25c89cfea"),
    hex!("ec1ba1810cafc7c0fe76e7bf50809bb2"),
    hex!("5ce691721774a58d63e53da2b80f0dbc"),
    hex!("f570455f0bfca4359608d92ba076c0cc"),
    hex!("1cdc79438ea2129bc739fc9497f53c14"),
    hex!("52bfc78f0fc5839e04f1c729c73a1469"),
    hex!("d711787384841b856ff7f4d53e5e42df"),
    hex!("63dd408ecfdd6e71d45cddfd45aff23b"),
    hex!("b0b09781e2c5249c9c248e0062a681ea"),
    hex!("0d9d5d09f198c9637b510bbac6f33f34"),
    hex!("b572f9b06f63d012d848174bd1191588"),
    hex!("d7ab790f4a80e62b38d3a8ae4d170832"),
    hex!("9184028922c8de7accdd9d72106aed6b"),
    hex!("9d52e83fb1ccb288a8bbd7094ea25221"),
    hex!("cb56adf452205662e1f83e51c0c496b5"),
    hex!("761eb4593abc7603cf0b5ea95d3661bd"),
    hex!("35ca47a1892c4524442a83fdc5231d3d"),
    hex!("289f4c7339489b0d07c8716fbf169c74"),
    hex!("75ec671be4712c1ce838fff26ef1122d"),
    hex!("ab650e5529ec2ce4147efe135a061eb1"),
    hex!("e0e637747620e8c1c0ef440b99eb9ce7"),
    hex!("94c0e63214f027f2ddd3ea463e44beb8"),
    hex!("8548626524a60410aee37ee400d237fc"),
    hex!("d80eb32a857a1f84b23801f6e4242459"),
    hex!("4853cb0907651c681f1dfbab0646a828"),
    hex!("ecd1edccd4844736d8a8e01d4ab21e59"),
    hex!("fb58a3ad252f9d576dcd1cfb23d32b89"),
    hex!("583b5070f416adbbf796976b2ca27066"),
    hex!("259d6fdcd7c3e46dd1a57ae64abda536"),
    hex!("d0c6caf2ce368aa85881e8c3bca18192"),
    hex!("7d54a3c9d517fba4ffb88cace0276c43"),
    hex!("630201121608bdec230db5d012bacfb4"),
    hex!("0da36e18ac524cab0cbd44ed0e70bf0e"),
    hex!("864cf4a44dfa1f5419a85613e03340b3"),
    hex!("d0369950eb82302e887caaca083d31b7"),
    hex!("2993e04f04c9b8476e92871886d88d7a"),
    hex!("dd49abb10a5bfaff4503b3a31874ac65"),
    hex!("96f5bb80bb703cd6b940b0fab926195a"),
    hex!("10e2c9baae90477c9be2f10365c29130"),
    hex!("696469c514035c0cdf657865a76c8b05"),
    hex!("e988c9b6348ae392d81e9d224c608247"),
    hex!("81a816b9971534a48e6ec21994b78c81"),
    hex!("5498cb9019ba94f896e2c04140cd036a"),
    hex!("17fa73eaa092e4bce97e3ba4b770a0b8"),
    hex!("e8c7b08816fc5215dfbe44cd46b47dec"),
    hex!("c30789092db881251b0c5f7373e0c6f0"),
    hex!("f397a1ac039c5e8bc374d1fd03568042"),
    hex!("33ec1f25215eae69085a3fbf7a6b27fa"),
    hex!("f6fdd17ce7427518d0631e269924f45b"),
    hex!("036c902bf005559ba3082e5f2201e614"),
    hex!("1fc45e655afc624fb90a7e0795b20b86"),
    hex!("bc94ffd5e4f606a12f0c0425d7bf1013"),
    hex!("21abfc7ec366c0b93e047d0d9d9df4bf"),
    hex!("b8a9f1c0b2d0601e00bb6fa35f3970e2"),
    hex!("d67fcb43ff2287a0cf8cf1f0a78ebc85"),
    hex!("ade2d8bdd4c48bd437b41d2a36424ef1"),
    hex!("d5550bdc493b35a3480c7a5f5d93e939"),
    hex!("b069c39e1059a068f9aa767b5a2c39d1"),
    hex!("e151a181c34b360acc4ae8f41f0eb923"),
    hex!("fa407454a0690b03f714c08ec72b3247"),
    hex!("10ffffcebaf525fbadcbe4aa46104680"),
    hex!("25569aef3173e2e81bd94a5e7904fc1b"),
    hex!("28681502310381ebc0ae31947c3cb188"),
    hex!("5db958abc1654596872a50938a0c9b24"),
    hex!("7c744e082a52a74767b70a72ec4489a9"),
    hex!("5b18ccdaa7efd9b3aff6bad60d547c81"),
    hex!("86322eab36c65090a3b7fdb5d7bc091c"),
    hex!("8423baac6908031fd9d08157f686b2dc"),
    hex!("08a1ade53581b4c029e1c002e51ceaf3"),
    hex!("f1ed7d196dff54c3421321acf939e08e"),
    hex!("2752d9c907207388e62373ed510c4e88"),
    hex!("c3c06fa841383ac60ccb91e4e05580d5"),
    hex!("a4c95f5a9ed58116110e43e663425608"),
    hex!("2c5bd140dff9063bba7ec0a206a3a4a0"),
    hex!("a5848a52ea19a2e85afeb598ce50eb47"),
    hex!("ff6279dc1306e5169f95f0b060e34b39"),
    hex!("da33c34ef46e9dd360b8dbe6531901b4"),
    hex!("83b7e0dbe63ffc49ffc59bae4b7b683e"),
    hex!("5c051f94fa62a73c11cfee276461fdb0"),
    hex!("798e3ba76c500e8177f392003ed1872b"),
    hex!("583d7265ee7126131854bbcb0de1f310"),
    hex!("90e4980b35640a8b3bb682ef2606e476"),
    hex!("6d431024b5bffd1270c0d041a05b815f"),
    hex!("496322b442254a79d1dd0dfdd6f51def"),
    hex!("92182683f38300b23bc0412e4138ac05"),
    hex!("212df134572585d10dd251f536025085"),
    hex!("63e2dbdb3937238a5d08cdf2b578b4e1"),
    hex!("96b819206e1d15573307e27b6ad290db"),
    hex!("0c54a577923b77c5a4ee726412c43be2"),
    hex!("155b53faed668b73ad702c93296a3e01"),
    hex!("896d7317a2f611e7363d93db93bcb72a"),
    hex!("a39c09d3a4ba25f3ce6691b85b390f3d"),
    hex!("7148171957df73a82553216488e35859"),
    hex!("ca049d60e60b7b69047e42f0b436ff67"),
    hex!("6f402a4a8208e9e49d4bf06f6ce7e11e"),
    hex!("95773e0c271ded0e10d2b47221c91e0e"),
    hex!("80fd5388433e89d3e74da2637216e3d8"),
    hex!("e35fe60581edd06fe880059a63952380"),
    hex!("24a5b87aba928ac920362a8bb3a853c1"),
    hex!("5a82f1cd0c0c58f0fbebb02c062dd029"),
    hex!("d8a989f4d05f65c07cd4f78d4c83d6de"),
    hex!("7e100ed69fa83cb97318cf268e063802"),
    hex!("5f7d7cb3363d1c4b41736787c8fa3a36"),
    hex!("03292bdeef76208a33368b1dd89c5f4f"),
    hex!("6b619e4bfd91e47efc4c6a18d6d2ddd4"),
    hex!("49e98cfac5039df5711f7bc82ca704fc"),
    hex!("bd17f87c484f37449d0cb26bee85352d"),
    hex!("b29204f91eeec3a61cf80f78d341e981"),
    hex!("0e2806dac2236f555aa1b60d44e6bb94"),
    hex!("84762739d031e5c2809951560a9aeaa2"),
    hex!("df1404d9feadf66ce9b6106bd730323f"),
    hex!("bf36c772e3f353b177dd77ff0af7f658"),
    hex!("c01a75724444ea62092d205d4f1faff8"),
    hex!("0eb6c4edf01055c26f19606f80660a82"),
    hex!("c5475e77e5b769f6e97f0aee53bb2927"),
    hex!("3a2a5f7f0ca0c8270800aa61bf75a256"),
    hex!("e2fbc1e07d14ac6e3a96cc9055750013"),
    hex!("226e5bbb1137417f87d4d0a638739739"),
    hex!("745c89d0db4461d9cf03e483f9ed2d66"),
    hex!("70ab39feaf98c852e8fac994ca8cc297"),
    hex!("cd9d7ebd5e7484375ec35bda9ebfad9b"),
    hex!("080de890fd9263b983b58e52f6dee214"),
    hex!("f67c8e857d379a60f7bf47b13ec08dc8"),
    hex!("b0b8ce46fdfa7f8b0091182cd9e52c19"),
    hex!("3fe2d70b44670254ddeaed4e46ba2d6a"),
    hex!("1e0f257e0107db4a3be7208c3490f3e8"),
    hex!("d0eb4a9ff0dc08a9149b275e3a64e93d"),
    hex!("eeab095cfa3a4dc8de4daf9c3e5affbe"),
    hex!("bee906bac51d709fa6c8d852834506fb"),
    hex!("85cd74d6633623e3e09d3b2ea0e8eebd"),
    hex!("f296dfe85523c5ab10cda4edaa513a52"),
    hex!("7d8ced87ed7fd15b2e4bbc0264e76f99"),
    hex!("ae69988dd1df0ff853e6ee66a5fe3210"),
    hex!("4469c4d95255369c6461be2862b915b4"),
    hex!("5709b43c1560bff7d265cfd850627680"),
    hex!("deb4f8617f931348359a3811076a30eb"),
    hex!("f881b9bdedd6f655e33220d24e1cc2eb"),
    hex!("ad903ea64fc18d570cd9a50e86bf033c"),
    hex!("4b3ac2630be5f8aab921697d1d1404bd"),
    hex!("07d5dd8bb48e7a72880b329cff744c4a"),
    hex!("84567d5b5e74e94c2373574d42ade1be"),
    hex!("63cf6b1ebbb29334730d8b9321cd264d"),
    hex!("83094b1464a6bbf92363619af081e20e"),
    hex!("7a93ae31b228b723301bf96ab9b0a09f"),
    hex!("16873ac9aead7c99286cce23dd91b4ee"),
    hex!("bf293be8af1eb38d7080957c7e1f8aeb"),
    hex!("967668d49545810fcf18632a5a3431e9"),
    hex!("475d5bbd6272a2695f66d2056da42bd9"),
    hex!("afc7e6ef08b5b8dc7a2bb1027160cd9c"),
    hex!("aa694f10ce796540ed77418cd9b35c86"),
    hex!("8be1f7a470d0c1edbbec6728fb0ff366"),
    hex!("7444078510fe6d9b3cf94188059a1366"),
    hex!("3739215eb46221b4040eea02c7757573"),
    hex!("a71b11286fff39e65eb3c8b3ac9a7219"),
    hex!("4b48bc59af9ddec38279e60178263779"),
    hex!("6076a0b6743690958cf040bfaefac391"),
    hex!("bead81dbb9227ba51a02f827f8dee2c5"),
    hex!("89508f9f01576f81853e8b92ba917838"),
    hex!("d075a5b5dcf20971f2e70e816bbcbb7e"),
    hex!("009554c550589a814909c9805279c743"),
    hex!("b470cf622846d536ad7b288b9074d667"),
    hex!("b87704373978613853240a3ec9368e8b"),
    hex!("7127b8d0e757abd6830b787afd829201"),
    hex!("f0cab8ea67e0a38ad606ab83ba6bc67e"),
    hex!("a408633718e44f4817c329af0395aabb"),
    hex!("4607a3ecef00a24da74521f22a6f8bee"),
    hex!("917cb60d42ccc40442e48be457f51dea"),
    hex!("90222d408a76f7f55fbb18282bef90da"),
    hex!("481d56afbd0ba6978e0ab2ada7b3506c"),
    hex!("604d874175bd36f8a02ce56b31ca827c"),
    hex!("6dc7717dfba128a330ea277dca94141d"),
    hex!("86226285351eba0c6e818826b1c562fb"),
    hex!("ae7280a5b84931846adff138820f221c"),
    hex!("be628492637e26e6489375f3a2938180"),
    hex!("7559678bfebb6f78e5c8026b17eadca3"),
    hex!("f38e7a19c004dd22688cf0079680bb1c"),
    hex!("c3b0e6a2b106f925aa2f92aac6213f8c"),
    hex!("eec733087a807a87a0c346de11513e12"),
    hex!("4c6d1ee77b414dc3bc448ecc0769a376"),
    hex!("303db177352ecf1920f09ba9fc8c6514"),
    hex!("8e38c47ebaf4ce8dc05178f3c5a9e86b"),
    hex!("104570237e9cbf0f4836ec8c4ff42f65"),
    hex!("4776ebe704f27086bcb98059906e8e3a"),
    hex!("c5aa722b23a6deef1d15a95f32dc4797"),
    hex!("c6188b4ee8720e1efa99aebeb02c7a67"),
    hex!("32701ac4e10f922048e0a7368e1f0452"),
    hex!("e5988223410c1d4f4260994faaf952b3"),
    hex!("2a92d9428c88e74bf47e545ea2025857"),
    hex!("04ca250a42e1f227955846abb768a035"),
    hex!("05b4a77d503468b71c0e730753fc1a56"),
    hex!("d7caf66b03181401cda1369c123d19f6"),
    hex!("6d3e29cb829b58d3fe90129c20dc9abb"),
    hex!("41b4f0817f11f8016023d74dea3eec97"),
    hex!("aeaa60d08ac92150b54908f7f8a92857"),
    hex!("c9453b8e185fb93ea0e1282e8803eff0"),
    hex!("e87f027df74563c88e700dfe057432ee"),
    hex!("af377ff39afc683033823eeb3ed0f10b"),
    hex!("f56a0b076a6bfc3eea7b1804b946d947"),
    hex!("69ba2470b6623fa3b9d68124e329513e"),
    hex!("575aee5f222f5ae9cca0973be3ad572f"),
    hex!("da97a6cd52c728a6f3bca987ebfa8cad"),
    hex!("4b5536ec8aad2250a2e38f6bfcdf58f4"),
    hex!("8fd3b4c5ad2c5743a6aae9f8219a60c6"),
    hex!("145b1a9812d684da23e74fead96c8552"),
    hex!("7617defe6ad9c021bc9bd7c809675624"),
    hex!("d9a2e97eaf84cce6294581acce315ed7"),
    hex!("3199b22620f39d534cd96fa8a032998b"),
    hex!("b1ca9b7eb944ea1f16364a1222b9afcd"),
    hex!("ecd0e506f3792f650fe5a00694afc356"),
    hex!("3b96f1eb7ad3124a51372cbe56f5c5e4"),
    hex!("962a5ed01d20d1202172cae5c4b1c7ed"),
    hex!("b5e9dc0e5c554931dba835dc88102421"),
    hex!("4596b31e8bf6c1f24b122de58efc7e1b"),
    hex!("224536fd41573a41daf7e131be8bdb09"),
    hex!("ef9661b2ac61737aa4bbba6fcad9f860"),
    hex!("26c9661a65164390de94c2d38c1f568a"),
    hex!("cc0b4699871953942cea3d167e8c9956"),
    hex!("575617f32549dc68ceb014b2f69d3b80"),
    hex!("932544c41c0e2d7af28189e513fb7ec5"),
    hex!("4b8e46de3ce76638280b9a699dfdb620"),
    hex!("53406aff68e56538b48fb98364e1a5a5"),
    hex!("928ae8d7116355d36b946a8182fc9923"),
    hex!("e30282bce7cdf44def0f840b6321e335"),
    hex!("beed3d40f310c0c6d0e18443f3304a60"),
    hex!("e2725bfdbac45fa18dabf0eb892f03d9"),
    hex!("07b43c42513772bc09aac4e471d67b16"),
    hex!("8609ba6e215f939caae8770e47d25f8a"),
    hex!("4287aec47a1da79aa2351f31cbd4ed0c"),
    hex!("b033cc4424fc38cbf7992491211c84c5"),
    hex!("cce1d898301da9cddb02d7f36181f8c2"),
    hex!("79e12de9d9e677ac2322705cc8a922b1"),
    hex!("c448a85e856037d8e88f672979a551eb"),
    hex!("467403ae25f597deb3c1094a2d33d413"),
    hex!("d7e03948dfccb6abb773409bd4a3c930"),
    hex!("674a8c75924d08965e7039c2e41f7940"),
    hex!("9220bbcb1742381fd5936662dee7210f"),
    hex!("505e4a4e5a49243957ee68bcf2ddb9e4"),
    hex!("85952e0b3c1032f7cad908bbd3a2b8a3"),
    hex!("f6e25da02626214f2dca471706a057d0"),
    hex!("dc7efbb16d990fb6db9e68efbc7fe740"),
    hex!("a3231a207b1daf19693a1a5ad18c6ac4"),
    hex!("90c5a0bbbc65a3fe44f2be3f860c5f0e"),
    hex!("3d8f53b6024c3b33b9097cc678de9a28"),
    hex!("1ad8cb3b8d1d4e04bb25330acd10b3e7"),
    hex!("c4830b15a969f30d1592527eda63bf82"),
    hex!("9d51b6f0c5be845ef775b6b900f0c993"),
    hex!("abdb6ff729edfa1fdf81725236fe166c"),
    hex!("f92a2b3fb5ebe93ee6fdac51e55f58d0"),
    hex!("bad463d68b2067ee099b35bc976d4262"),
    hex!("8a326abf1bf139fd19a9931aad716e2b"),
    hex!("21a32ae99babd87319e21b115291fa93"),
    hex!("aed51baf66ff4910f3b84c6dddd277d0"),
    hex!("65c3bbb3015925ae57d939a67bb3e1a2"),
    hex!("97bc9538e14c7d221d3fba271fe1a9a3"),
    hex!("6394e2557149a2acf674610e834f02a7"),
    hex!("280dcfe6935188046eefb81a77e043db"),
    hex!("313d0d27a7b82f6e85b32037b3458025"),
    hex!("af7416b95834809dc8619c24d9f70575"),
    hex!("9e14b1882ac75f1b7ac8735e89bd1dcf"),
    hex!("f770f4047a86f36727fcde69c0cb8b68"),
    hex!("004610125634efd77979c429a95f16e9"),
    hex!("9fb78c563cc2617353fb943c5c6029d9"),
    hex!("addc6c96bafb15254e0e2c2a21f6eca0"),
    hex!("b2e1d71c4419cf35d2ccb202727e9006"),
    hex!("22c2cf6192e5f767d518ba32d2628f27"),
    hex!("d4a9a8dedeaa916c20451f72d868e54c"),
    hex!("e15c7e3a6935f188aab577be046518f8"),
    hex!("d00f06b2b19fb192d885586001624318"),
    hex!("3c1133d7e7085944fa800c1365d4b4f3"),
    hex!("3963a16de74721a202e7f10d66278fe4"),
    hex!("2f886a0a39058911d72b46e15bc34672"),
    hex!("bf8c454a96a689eb71c30d9639aaecee"),
    hex!("761b3e46118bc24bc62987107f3d12c6"),
    hex!("891583dc69ff4a5e64070d942aaa435f"),
    hex!("d8b34532a52763f1afd495aa3e36b2ef"),
    hex!("2f9e4d03913cd937e09c451b3ed20dcb"),
    hex!("93d22323cd8c06ec945733ee811d8ac8"),
    hex!("2a9d9c385dc260a178c9dd5902499f7e"),
    hex!("45e79066792ee537ae6106b3c897d44c"),
    hex!("4e00df4f849deba8f05284dba1a8daf6"),
    hex!("9ed2f8a53f69dee1e9b2d4a332ac80d5"),
    hex!("b0cb763b4c0e4bddbdeab130195681bb"),
    hex!("c25c64f479521ed7a68cb75637498e67"),
    hex!("a66e88f5a0279ebbfc9063d5d7fc9681"),
    hex!("97f23e83e5a2c1e6209a1e0baa4c9048"),
    hex!("08efb5ef7d86b52c486f88ea92865e2e"),
    hex!("750b98718c4d7f9b63a0fe4135a00143"),
    hex!("bd71d4d32938661a8e4e8e198f6e3c71"),
    hex!("dac6dce2e49f253706ee5ea4549abb67"),
    hex!("1dfa7fc8cff2108f4de96a6f6404321b"),
    hex!("58fa94796612dacc2f2a60fbac5f85d6"),
    hex!("af4a599a7afc59244662fb56a32f38cb"),
    hex!("7b2920aac8c076c5fccfdf3325fc8455"),
    hex!("b3328f0b1057958da28bab59330133a7"),
    hex!("ad4e0add9ad103421f47d88eeb5c711f"),
    hex!("4825b9d42589e834f61e6ef705641713"),
    hex!("3da44d4f1d8bb790537ec42ba2af168c"),
    hex!("87db7dab6b1aa2857fcf861273b9a58d"),
    hex!("c32c902e1389ebda24a09ae882575370"),
    hex!("cf17c3f198e852d5123942c402918656"),
    hex!("9f1cf97072ee00922c301340a19c91b7"),
    hex!("b3e163f4cbeac4437a962c84a85a1e5b"),
    hex!("a70314ea9655ebf03ee78a4a320d1ecc"),
    hex!("2ab485395195fd37e0fd5b2336f0a00a"),
    hex!("9f77060b503e1fbccf8b682215821b07"),
    hex!("a4fd17b615f2794b3fbb98ac81e0c5e7"),
    hex!("3e7faa44b3e919bf089ce8962a41596b"),
    hex!("f1cb06f527cfdb2bfb3e3341c878101d"),
    hex!("fe8cedf87702d7b090a0f07571607d86"),
    hex!("f569a8f30771d73544ad99fb1610b174"),
    hex!("1e332a7f9b33fc91369ba33503353023"),
    hex!("e04c52de8e81749474a0a3ef746c4c9d"),
    hex!("e961634b1721573ccbaf4c195ece7bd4"),
    hex!("c50b42bd793d49f0505df93353c4acde"),
    hex!("f8a9ea7fd860ad32e03ed50aebeb92f2"),
    hex!("f6a622025cb1659a5bce3c4cc7ed0680"),
    hex!("b6a78250c0253c2a8a985beb3ed16309"),
    hex!("d2ba47f421049058107969e08458e7bc"),
    hex!("66809b4880f156c8f539441829d11b90"),
    hex!("980b88f3b17ad1bf46ddc89356df550c"),
    hex!("083177d975088d3b3acb85c5e767948f"),
    hex!("07a3e31da3988ccc22a48cb61890ed83"),
    hex!("12c4f7a7402ada8fac7c2ddc784ca2cb"),
    hex!("a7bd8cdd867b4b3812f3066b3db3c006"),
    hex!("aa098d01c41cc948c138f864a8a62481"),
    hex!("18457233e28062083f7d23b2e481189d"),
    hex!("1702cda0b76772ba09cea0edc5e5746e"),
    hex!("db200270afe9e05cba79d94ff6d2da8c"),
    hex!("b93ce415bb6beb51157141149e34bd0e"),
    hex!("6266741ef0b85a2fd5ac4a1fb816835b"),
    hex!("8dba28245cf055574881b05fef9953a6"),
    hex!("e4af90f7979c2c631633131d642dd8bd"),
    hex!("97f98f4275be120a445cd0275e2cd73a"),
    hex!("150a9c0526b11752453a23d8b18a8f3b"),
    hex!("010bbf6895ade2375c8478a0c3151ce5"),
    hex!("355796530fdacf6d87bcc370f17fc71e"),
    hex!("9a404317c26f415ed025f32dfabe8598"),
    hex!("15d2eb783afced72c733f6ce90bf7349"),
    hex!("fb9f445a7acf24b91e6cbe8f9489a7c2"),
    hex!("6f03e5d4ef52a7c05a5a5fd28b159b5b"),
    hex!("2466fb6d4eb8aa1c700e728fded218df"),
    hex!("676cfafe2fbcffd070ddb236d2bb0021"),
    hex!("91e33a111622283750412eea13c83f35"),
    hex!("88b1f25057c3bac8ee1eeca2ff2209a3"),
    hex!("c10d6e9c953ebdc8ece36c5cd6223387"),
    hex!("1fb01164b818aa63387a0ec14be5e3e7"),
    hex!("aca8367a8bfd04541cc836e293255b77"),
    hex!("8b74b13c0d49da16c37a8de608c18e7e"),
    hex!("79e4197b401889e0756cedda74f46812"),
    hex!("fdfc1643dbd6ad08bd6a4eba37a0e3c3"),
    hex!("3c4b6a74dd034b4e72bc84652a09a3ff"),
    hex!("2f31fab52ef05919d280c2abcf422fab"),
    hex!("4a2f98048e8605e4d439ff8554ab6e63"),
    hex!("3b7e760d63c75a4c368dd53425084427"),
    hex!("dbd55facc2eed4edae760a2ba92b4f39"),
    hex!("43b123b7bca43b561fc26e423bcef939"),
    hex!("b6305a7b627bc5973e579f6984661e92"),
    hex!("ec1c177be3ea3294f799a7431bddc5ed"),
    hex!("db89f09027441e9465a797737c5a647a"),
    hex!("56dd88789701e0e1c682d8ad251dbeb3"),
    hex!("569403a8edac9edce0cf1e8876c53174"),
    hex!("7713e55406eb2a9076398f94d0a58692"),
    hex!("c1f88b2f71fa2e1a988038cf9d3df04f"),
    hex!("d81ef241ea5de4b34aaf39ab1b083642"),
    hex!("d375d23cdd8026becba44bba2b294c3f"),
    hex!("1b7eee6c46118d885bcaaaabb2f9badb"),
    hex!("89558455a420ed8268a592b0eaf204c6"),
    hex!("b17d6916a9d0db09432db599e90327d7"),
    hex!("e1b36f36682b4fae32da6979590cc499"),
    hex!("612373badc313851be94e09ccff61c5a"),
    hex!("faf60883e0085e9672e6521bcdb86f5a"),
    hex!("7784f2c8187dddb17dc42deeb335b625"),
    hex!("e53fa1a5071b726701e5eae987891d4f"),
    hex!("5c8291e6ac0b0abeac024d54547aec5b"),
    hex!("6462d16fbb1f465357418796773abc49"),
    hex!("cab6e515313f84de7345574d91480771"),
    hex!("5f64f987085767b41be43f261a8ba025"),
    hex!("d6c688f958dbe3b9a333041c25e067f9"),
    hex!("80f05a86da8eb6a002c54c1305d5dc21"),
    hex!("60bfbb201ce2fb723edf4a6e2433b6d7"),
    hex!("ebc37a445da40d345532a1b9bad5445c"),
    hex!("d47c63b0619011b6bf65c269a047def5"),
    hex!("1aba33a3d2b769a78bd6ff673be3b632"),
    hex!("04b7d271316b4c31a58f21d31ddb5fd0"),
    hex!("3226afaecb913b1380a2c4220efcf329"),
    hex!("dc0605dbfafa3292db3e030284dae0c9"),
    hex!("42cb016d792a792545f5d3628d280b86"),
    hex!("209c20b1a337f22459ddfe9cb9cc5616"),
    hex!("f136ac0b645a83514ba9e2b5dd6b33f6"),
    hex!("93d1365bf44fee8d78521c97ba8fe6a5"),
    hex!("9500fa38f25ee9bd5140db4f3f9ca585"),
    hex!("3dc470c2bf7dbd809c66c922c31b673d"),
    hex!("d03478b8608f6d68335d25256c9ddecb"),
    hex!("e696ad7c49a89b837a68bf8aad0670e6"),
    hex!("8c4a01a4de7453ea9d5e01851b68c624"),
    hex!("0064ee1b40e9947a06f5692a367cfcf7"),
    hex!("44a780519b6751d1ce84c01dfd26919a"),
    hex!("810e0008a42674b8503ccee1e487359b"),
    hex!("14f37bbb397085efd87ca8d08f73745c"),
    hex!("a4531df9fb35ffeb48f433aec36d2644"),
    hex!("ebbcecd01539dbbf24f6f918afb8c8f5"),
    hex!("e594f29d5159f01eb37b4de961edf17f"),
    hex!("435fb4690bd8681ce2e7aa577fe88428"),
    hex!("c685712dda4ca735a9336290c19e6757"),
    hex!("76fa7c02c91a2acd4feac2c8b7e79241"),
    hex!("6ded6bbb2b6ac1a8bf368ec35f8c0004"),
    hex!("853104204b4e7dd21f1891a57dc99bda"),
    hex!("016c939b9fad1bc914827b81b63f8f8a"),
    hex!("0aad21f534784ce6e55bc860be909ff8"),
    hex!("d892c0ed69e87bbcb0b75e954629415e"),
    hex!("6e00495d56c440a0bfe596a261856dc3"),
    hex!("12c24ed58e08eb9d84123af23954338c"),
    hex!("b253ed770e7a12a526eb55524e9b2d78"),
    hex!("867143a311b72df9301212469856d6e8"),
    hex!("30645981186d86d7f453b627474bb186"),
    hex!("628df7191dbce185c3894f83b4b100bc"),
    hex!("abf05f494a51a66d0be185be6f9b55ab"),
    hex!("23b6a4e817837ea9bc1e8cf4d12432b5"),
    hex!("eedbaa71b581216e1bb14b5b6e370cfe"),
    hex!("6571c261d0d1d9ac6ec81ef3c5579738"),
    hex!("90ca319d7900ab33585929f87f87f4f7"),
    hex!("3aaad6c72aba1f9bbeb03ddca6716189"),
    hex!("e2c1d3bd988d38bc6baaa7572d4b33a3"),
    hex!("2b06afc482dc2a2814244c0ddb85172d"),
    hex!("311d1fdee66191a759777222b885042a"),
    hex!("d32037774ed016be27a05110188f33f5"),
    hex!("8bbe9d2540a8548c6e837c1ca1be1736"),
    hex!("97d081a75c821656392370b91f0551e7"),
    hex!("6cdb7e28fad3da3419f3bdbfda3670ef"),
    hex!("0eef01ab63f2fea60d800a3e50ce7f4d"),
    hex!("26de913e72381b29ae92553c5dd56026"),
    hex!("c010b7c20f4e8eb014c9e14ae5a2cfe7"),
    hex!("36349c7a4f4419889ac51938e01dd562"),
    hex!("62a66c09d10bb962a9482a274fdd1ad6"),
    hex!("0ca8993c5f9e9441ee476186e825fd02"),
    hex!("32720f4423f0c648aa43f00eec2faa7d"),
    hex!("a5a0c7c7cd1643d0ed59990e9aaab8e1"),
    hex!("75a4421dc20eb16958d5cc74d0018376"),
    hex!("d18f3989a5ad09f60559eb953918ce96"),
    hex!("bc1b06b4746555e1da474d99578e1bd8"),
    hex!("c156a24e9e9461df6a7b38135e15251d"),
    hex!("a86706531ea8351a7ab7e346af0f29c3"),
    hex!("1e09fb83c81c2e837ec8182d573fb9d3"),
    hex!("6056b80049473d253538113d651825fe"),
    hex!("48aecd0743c8149eb5f9aec60f347f7d"),
    hex!("a028808b3199d97d3fc40436bde64c56"),
    hex!("9c84842024616e12f5733c3c64877d8f"),
    hex!("61a39835bde221a5e1ceef6f92b8ab78"),
    hex!("a9705493cafe59c707acd9d03b91f648"),
    hex!("0bc0761a4c3ec1f871638dbdd7786fe1"),
    hex!("a798e4c2b322a4e64cbe4a1f86ab3721"),
    hex!("5c36cd452450b556692b4958b5c64d2c"),
    hex!("fcbcc2f7cb02c16981037e57c8a6aac6"),
    hex!("68b580e09ca06432272557e38d2cb250"),
    hex!("8166a9706fdf02dfb2548852b0080cde"),
    hex!("f5bf78f8ea50a5cc718e8cfd22f96013"),
    hex!("f78d251520533f415055ec2f705d050f"),
    hex!("c4ae15a70fa1e873e3ea7c747b7eb2df"),
    hex!("219aea4518c67efe0389e5b3eb083d3a"),
    hex!("92eedded99e1bb261e106a9d23bb6b97"),
    hex!("e6c5f02b94e81da315deffa12bbf5f8f"),
    hex!("beea81cbe3ebf59d893c325dababaa8c"),
    hex!("d838adeb0ac44aeb59d09eede9074d14"),
    hex!("fabaa47e5558692b9b17621a26549da7"),
    hex!("ef9da8f942af3ee8bf2803c65e25e51a"),
    hex!("200211848e159a56eeb6cec602ed583c"),
    hex!("83b04a1330ad921447fc3ded47ff75f0"),
    hex!("cdba71a13a331d9fb32e16e103bd5dc6"),
    hex!("6d261a84b2e3f0f45f945982000a07db"),
    hex!("f607a8685065a531fa2a7198ca2181b7"),
    hex!("7f636cf604a13da02fc30d632feb1088"),
    hex!("717a02b8df54a19dc814b7fc875ecfdb"),
    hex!("828188a8255344756d3763267c145ee6"),
    hex!("7cf82d8169205840fb432bf99543f39a"),
    hex!("47409b2432ff64f26c16f3bafccacfc8"),
    hex!("b0266545e832b14a2a70aa9f64bfee3c"),
    hex!("a27425aabc0a1193e41c8a25da150268"),
    hex!("f631b7ec97e689f418d953ab50c6eea2"),
    hex!("cdd6e178da5a864c0f4efcaf2f09a7a1"),
    hex!("8dfb54b631c611bdd3d9b8ea1a139351"),
    hex!("75c92c082a8a3b145cae31cf00f84fa3"),
    hex!("7a9010568819de327a24fa495029adcb"),
];

pub const ETCHASH_DAGS_MERKLE_ROOTS: [[u8; 16]; 145] = [
    hex!("55b891e842e58f58956a847cbbf67821"),
    hex!("7f273e28b9296b6d6d07415670ebfb73"),
    hex!("b5b03b1c4010e3fdb51b20300f10962d"),
    hex!("67d5d9b5c8e6c9b9ec3a69066709fc19"),
    hex!("5670c73868373a41a23c9df6fa34341a"),
    hex!("5071ccf3d27d915a115f70ceb494e546"),
    hex!("fb0bfb6013366ad6ce66f01f3ddabae8"),
    hex!("a7e90b4846e9e6ac28118bf7d06a7dbd"),
    hex!("4d104a1ead35304308a279252ae39d1a"),
    hex!("902ccfeccd92fdf73c6e45d5fd2a5c2e"),
    hex!("acd51dd3b1dd1886c88c6f7d7e99378a"),
    hex!("329ddce3ccec7a018f5853458cd46a7e"),
    hex!("6495bba666e2da9e2954594c9ae505d6"),
    hex!("52beabff58be01a5f36a09d3e6e903eb"),
    hex!("4c78423e73e050ab6c7d787827a2a5c5"),
    hex!("725cf863a8e128c24f075f558d02c701"),
    hex!("6ad88f9413e72e5c642c08653cae161e"),
    hex!("605b9f7be2de820bb8303ec51d633035"),
    hex!("5d876c503ec4daad1d141eebc099f47d"),
    hex!("749d8ca14806131baf62e6dfbdcc7273"),
    hex!("ef87a266321fa67abe1cd348b9ad5614"),
    hex!("0fc61f73cc03c2bf6d82f25725283da0"),
    hex!("0ef54ff92e9c591eaccb6a92cd8e967f"),
    hex!("ea34f6f8388049e42d19c7f64dcdc8b2"),
    hex!("b75aa317b1ace2181014bd1d519af5eb"),
    hex!("df3a2231fe6a147acf80fdd5b57d1e7c"),
    hex!("36d0cd0b2842e0d989020c2cc00ab3d8"),
    hex!("de366c6aa69a212da2b5235fdf9841e8"),
    hex!("7cec28718e88053baf88af1933e6a64e"),
    hex!("40d162f15c5cdaa7d127d15cdbd9b055"),
    hex!("5cf1f457f8f4ec5cb6eafeb72b9807a8"),
    hex!("de2f341d96d560ba451c9745117bfe3a"),
    hex!("6030528bc9c4a9f29fbe54e383d97ef7"),
    hex!("cdec1db0db6fddc666aef77bdec7619c"),
    hex!("ba21ee19abb476ce55696df460dba7f5"),
    hex!("499962fa8fd1df5ace9e3eba09591dc5"),
    hex!("0f2fa1d79ff5b6d733596fb3b39a3c7e"),
    hex!("485e8cee8c3410c1898311184fa2ac81"),
    hex!("27b0b7cac7fb34e5741e6d4cfd96dd00"),
    hex!("24f4b6c860f1f15553392534b7e73682"),
    hex!("96eeaff668a0894cb8e4929c389bef5c"),
    hex!("b1957e513cac0af7fb72fe4690db02ac"),
    hex!("2b1172b50d5d4bf8d626a2c187f6c0a6"),
    hex!("6245d750e10cc35f0814be044e43c095"),
    hex!("8cff907c7e4f3f6c3eb0803f34945d52"),
    hex!("1829929d94f6bb96c1731fe518abe51a"),
    hex!("7a2e85bb3f592db5c60b1224c72a0783"),
    hex!("0d7fbd40e0079f10dab1f9963ef49119"),
    hex!("8e1d3fa407ea406c7eff92447ac8dda1"),
    hex!("86af0208ad57a818911ae07a5f80a3df"),
    hex!("0ac55665aac060d5837b1b7d1aec1525"),
    hex!("b638d6df79d41fdf1d3d5a7133a76fc1"),
    hex!("96ebb471f6b6b51df71107a45dca78ea"),
    hex!("3b9991b61fdb45d5190fff5381ac4588"),
    hex!("4f907251cf47ab88f56ffbef4ea8099c"),
    hex!("d52ca6657d33fc13507076864c7e41ae"),
    hex!("f95d2ff7841aaf548eba2c07655bccb4"),
    hex!("b5d35dde4c39796d194e1b7849743c4e"),
    hex!("7edac496e3f80cd5f63283c4d75a4244"),
    hex!("dacf2f1cb07054e7a2054a09c204d04d"),
    hex!("86e6cbbb595d0a9055c71649f59f6ee4"),
    hex!("a2d107670aabfd2cebdba942dc4f1cd5"),
    hex!("67dfbcc82db89b0552b0b712defc4b20"),
    hex!("7d2e941bff5355cb60ab85c32c260348"),
    hex!("4061289609931f8eef6db323b64f4e1c"),
    hex!("6f8abc4e5cf46ac6b85b68a74b6dcf96"),
    hex!("899751eca65f613331d3baac301e48c8"),
    hex!("80fa9e701257a5688c7d4d5bd431fb7e"),
    hex!("a235fc7f45656adff0b0d77ad313619d"),
    hex!("c05934de29de5311a6fe9ea4f0928af9"),
    hex!("72044ae63e149134c5920d78276b1ef6"),
    hex!("d22e494bb32f392e7af8c61c9a4dd0d7"),
    hex!("69372ba5a14d95bc4ec2ce347c2f5658"),
    hex!("c38ebdd4ecd2b9f526389ae7cf05131b"),
    hex!("904a403aa203735391ae1931b2964da1"),
    hex!("b0d4f533938a353952245578de227b89"),
    hex!("af3dfade4b696fdc8502dca6a20638e2"),
    hex!("40b114689cafd2b37c86e4e5e126732c"),
    hex!("bf48839b9a1db12d60b495c4503d9248"),
    hex!("928bd14afba09dce651a6408b5c40d4e"),
    hex!("a94508d41a40dc3ccaa5cc0c4c743d13"),
    hex!("5707fb663a7ec512617c5df295b5c9ee"),
    hex!("26cb8f232c5037f9c17bfdb92fd852f5"),
    hex!("04f2240cf02105776d00f72b9417669c"),
    hex!("7f2c9d236a54fd3f52d6e921fc8695e5"),
    hex!("648556fda12ed6661a82207ce569e4a0"),
    hex!("6ebe99d910776aea563aec189119dfe3"),
    hex!("eab19714637d49b4e5bb93e1fdb68df5"),
    hex!("5fc7a8028bd5e985e98b8a37c7718c5f"),
    hex!("5d2c763f8da1d62bc1b48e422a5ca968"),
    hex!("f11c2b21eedd7cf21fe2d2538100321c"),
    hex!("2e0f114f5ab3140ea1367b465cb0f1cc"),
    hex!("551ed99e9dd9a086fc2da58a80be12ac"),
    hex!("2ed46acc287d46f225639bb99a02e0d3"),
    hex!("8e61dfe10a0ecd3dfb82f54f054414dc"),
    hex!("e28f6d6f826332c2a14a232aad443af6"),
    hex!("370ad92c5c4877a379f6512786221772"),
    hex!("910291c669a996aae104c1ab7505ec0d"),
    hex!("1f0f73dbb4ff3656312581048495d6a7"),
    hex!("803fe8e6f436e4c7e7af4a43bc1ecba6"),
    hex!("3ac7bbadf608a503621c6ca6b8f0cbb1"),
    hex!("fd941b54744d81163ba4205a8fa86803"),
    hex!("ca4275a1859a1cf609f730a615f356e9"),
    hex!("efe35da46392cc4b908c3f646948eb86"),
    hex!("7ee93b5350b60f5907ad81378c223b93"),
    hex!("71ecc3c8ece7840b4770a17840ba2872"),
    hex!("291b43bc85ef9c546bbbe06f40c779ce"),
    hex!("991e2cbd87d13ac03723391ae3dcc21a"),
    hex!("29537faee353c0b6e96426930056d937"),
    hex!("79b545f718f3c37ffef35d3bffd4fdfc"),
    hex!("2b4e8afad5c0fd41fe69187bbdfccb30"),
    hex!("e552db6af978f6547d23911576df888f"),
    hex!("fb95184fe37234140ab43c3e3da55c53"),
    hex!("ff7bb9057bf5b3de6579b36d3deabd1c"),
    hex!("380ea8525d71514156f598c082261d59"),
    hex!("16e5378f96c77e6c63fc5c43bcf6bf51"),
    hex!("8f6687342d5340e613c988de96724984"),
    hex!("951e842c93b942659e5e0447f4a0c2cc"),
    hex!("a7e67d7b0f2a41c6f3b27de2d3d197ca"),
    hex!("60ead07ba6abf74df46802a0573b2c76"),
    hex!("5aa8f2821e0735c230570ff01a3ff2e6"),
    hex!("32d27df41bf796479176db6b78ac5182"),
    hex!("b401095490933f281dbdbb9c0e829d3b"),
    hex!("e52275f99d0651985ceacf85ad48c226"),
    hex!("09d642ed191c0ef4a959eab727c39719"),
    hex!("6ff74bc96886f1554bd19fc675904244"),
    hex!("475d7b475e8bd1e952d9bb9f6d5342ff"),
    hex!("88c5091fab30abdd7349f20d49b61b63"),
    hex!("254d037eed76c6dc55175d4c1e063adc"),
    hex!("d4bcb489fb34d3b7d44ba7c3834c31d3"),
    hex!("afe4ffd6fb7260f8c058b171f4692ccb"),
    hex!("dec35d9e9025b9d915ad4c05b9f7c49e"),
    hex!("2c61250e2de35703313bc0fee2519560"),
    hex!("c0d7a3e60488a1723229123a4ae7a1c6"),
    hex!("1b23c2e115b2cdebb35406c078f53c3f"),
    hex!("0fd0252a0bca60e46df968d858d68500"),
    hex!("3b0aaac37371a6a3d914627633a30f5c"),
    hex!("202f0be4de4a38c9e2483165b52b4b39"),
    hex!("98d3fa868630f09312c22d97f51fc22e"),
    hex!("03bf0c295a3e0de88994ca93e77d49a2"),
    hex!("4a4f569d38903394a6ceaf9e5d13ab74"),
    hex!("d43ab9942500ad16a0296f512718050a"),
    hex!("652d690abf25ea5a8432ecde408f8b09"),
    hex!("c8044278781a10af8b49eb3eab91435c"),
    hex!("9f6e211fe8abfaba20b5a4d352b8e3d6"),
];
