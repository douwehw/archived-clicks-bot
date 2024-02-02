//#![windows_subsystem = "windows"]

use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::process;
use winapi::um::winbase::GetCurrentHwProfileA;

#[link_section = ".code"]
pub fn hwid_check() {
    let mut info = MaybeUninit::uninit();
    let hwid;

    let hwids = [
        "{e0ec89c0-2a3a-11eb-ba6e-806e6f6e6963}", //alpha
        "{b17cb4c0-9acb-11ea-8b08-806e6f6e6963}", //fire
        "{b45c8865-4a25-11eb-b562-806e6f6e6963}", //krx
        "{a81c14bd-8ff5-11eb-a21c-806e6f6e6963}", //deactive
        "{846ee340-7039-11de-9d20-806e6f6e6963}", //doki
        "{7f2edac0-ddce-11ea-a034-806e6f6e6963}", //knali
        "{2e9f0c72-00d9-11eb-a22d-806e6f6e6963}", //phobic
        "{0a0c1ac0-2a39-11eb-9773-806e6f6e6963}", //inx~
        "{6c3344d3-90db-11ea-a71d-806e6f6e6963}", //damian
        "{ac0401f1-158f-11eb-9084-806e6f6e6963}", //swofty
        "{7bde8d40-2138-11eb-b915-806e6f6e6963}", //yoshi
        "{e0ec89c0-2a3a-11eb-ba6e-806e6f6e6963}", //savorclaw
        "{00299a44-ac50-11eb-99d8-806e6f6e6963}", //trey
        "{84eb96ec-4408-11eb-885b-806e6f6e6963}", //aet
        "{a98a35d0-ccdb-11e9-908d-806e6f6e6963}", //ryamu
        "{ed85dbef-1ed2-11ea-87c1-806e6f6e6963}", //keshi
        "{da7c0897-c4b8-b2f8-7ec0-acb8212de7b2}", //wolfzz
        "{d73f6552-d8e5-11eb-baa4-806e6f6e6963}", //fluyd
        "{115b63e0-3924-2259-4445-04eb082f1640}", //talan
        "{083ab41a-e13d-11ea-9786-806e6f6e6963}", //fnm04
        "{15a7418f-d929-11eb-9665-806e6f6e6963}", //tomson
        "{ed93c02e-3d4d-11ea-ab87-806e6f6e6963}", //Laq
        "{083ab41a-e13d-11ea-9786-806e6f6e6963}", //fnm04
        "{Apple19910-7183-12265-1988010294}",     //aiD
        "{3d6126bb-ddb6-11ec-b81d-806e6f6e6963}", //0pacity
        "{d2d35a40-00d6-11eb-923f-806e6f6e6963}", //ptyx
        "{0a0c1ac0-2a39-11eb-9773-806e6f6e6963}", //quake
        "{8bd75640-29ef-11eb-acc5-806e6f6e6963}", //naxio
        "{7eaf9347-3b22-11eb-9161-806e6f6e6963}", //m4rzl4zer
        "{ee17b709-1369-11ea-b61a-806e6f6e6963}", //bease
        "{92d6806c-d125-11ea-9a8c-806e6f6e6963}", //caprisun
        "{3bbfb2ca-9879-235d-f9fc-87c04ae91321}", //sweet meat
        "{59cd1bc0-d793-11ea-bab0-806e6f6e6963}", //jolly
        "{fa635d26-06cc-11ec-87ac-806e6f6e6963}", //dray
        "{1bfcb4b9-c865-11ea-853a-806e6f6e6963}", //haxorras
        "{677c4810-73c8-11ea-bc07-806e6f6e6963}", //jj
        "{e1ec9c49-14eb-11eb-b096-806e6f6e6963}", //falloostic
        "{b9337d0c-cc42-11ea-9057-806e6f6e6963}", //ll
        "{abe44e92-7f06-11eb-b119-806e6f6e6963}", //zregus
        "{8c4b27c0-8a63-11eb-99e6-806e6f6e6963}", //vernam
        "{e31c90b7-273d-11ea-85f3-806e6f6e6963}", //blitzen
        "{970a593b-c840-11ea-8505-806e6f6e6963}", //r4v3n
        "{2fe56740-1a19-11eb-bf35-806e6f6e6963}", //lolxd
        "{f8dcfe3c-9240-11eb-b09c-806e6f6e6963}", //alyn
        "{2a4b9c46-b1b5-11ea-a621-806e6f6e6963}", //neon
        "{06dead40-6d50-11eb-86b0-806e6f6e6963}", //pyro
        "{0af424ca-0963-11eb-a4bd-806e6f6e6963}", //grave
        "{19b056e0-090f-11eb-8347-806e6f6e6963}", //callsai
        "{d0c365a6-7985-11eb-8b50-806e6f6e6963}", //laff
        "{520fad40-4317-11eb-8f51-806e6f6e6963}", //ewoks
        "{c24973ed-21df-11eb-b4b3-806e6f6e6963}", //femboytoucher
        "{497384f1-b16a-11ea-b442-806e6f6e6963}", //vantablack
        "{460a9fbf-b4c1-11ea-a1af-806e6f6e6963}", //naomi
        "{5fffa4c0-29de-11eb-9669-806e6f6e6963}", //laens
        "{1aef9803-e153-11eb-81a2-806e6f6e6963}", //koishi
        "{2660c64f-7607-11eb-badb-806e6f6e6963}", //anywayev
        "{13f83386-dff0-11eb-a45c-806e6f6e6963}", //exa
        "{3131028a-e586-11e9-9e9a-806e6f6e6963}", //wes
        "{3570a2c0-67e4-11ea-bd4f-806e6f6e6963}", //hipo
        "{477d23c0-8748-11ea-9ef9-806e6f6e6963}", //cyrekt
        "{1a4af255-23f2-11eb-9281-806e6f6e6963}", //awa
        "{a0ae65e8-6cb1-11eb-85b2-806e6f6e6963}", //alez
        "{cc4a2702-705a-11ea-a4d5-806e6f6e6963}", //dscent
        "{1f15070c-bd44-11ea-925d-806e6f6e6963}", //louis
        "{5c7bbac7-ec11-11eb-beba-806e6f6e6963}", //louis 2
        "{a485bee9-0167-11ec-a1e5-806e6f6e6963}", //louis 3
        "{a5cc6074-bde1-11ea-b452-806e6f6e6963}", //blzr
        "{b219c49a-af87-11e6-9c01-806e6f6e6963}", //orecreeper
        "{aeada51a-9096-11eb-8f90-806e6f6e6963}", //edz
        "{12340001-1234-1234-1234-123456789012}", //doki 2
        "{2b93df8e-70f7-11eb-bd07-806e6f6e6963}", //patrick
        "{d252b8f1-5cb3-11ea-9868-806e6f6e6963}", //exen
        "{cd4f5440-0d13-11eb-bdb8-806e6f6e6963}", //juuted
        "{f1f40dec-1b52-11eb-b908-806e6f6e6963}", //randomb
        "{2e9f0c72-00d9-11eb-a22d-806e6f6e6963}", //fracture is hot
        "{ccd26940-29f5-11eb-b712-806e6f6e6963}", //betcha
        "{91ed1ccc-fafc-11ea-abbc-806e6f6e6963}", //endlevel
        "{1192f2da-7637-11eb-9ab3-806e6f6e6963}", //souptime
        "{a472aac0-0374-11eb-b30f-806e6f6e6963}", //cringe boi
        "{02a751a8-18c0-11ea-ad63-806e6f6e6963}", //clown
        "{bbed3e02-0b41-11e3-8249-806e6f6e6963}", //krij
        "{54f5bf25-88c8-11eb-a338-806e6f6e6963}", //cedric
        "{eded3019-85f2-11eb-9ce4-806e6f6e6963}", //noire
        "{51480c12-8c2a-11eb-a4b7-806e6f6e6963}", //zlq
        "{0b10ba95-b4cd-11ea-a02e-806e6f6e6963}", //drumslayer
        "{3cbe9621-6424-11eb-8f35-806e6f6e6963}", //RTX
        "{1564cd79-dcd0-11eb-8069-806e6f6e6963}", //phipan
        "{6f8cb340-dac0-11eb-ab2f-806e6f6e6963}", //macha
        "{22a602a9-8d14-11eb-9d67-806e6f6e6963}", //saturn-
        "{6fca8781-d0b8-11ea-b354-806e6f6e6963}", //zhander
        "{e6939491-ff1d-11ea-8296-806e6f6e6963}", //peeb
        "{ee7b2728-cd04-11e9-b5c6-806e6f6e6963}", //paqoe
        "{2e9f0c72-00d9-11eb-a22d-806e6f6e6963}", //cheatzy
        //"{1041e888-1108-11eb-a607-806e6f6e6963}", //mr_games242
        "{20b9b1c9-fb9f-11ea-a536-806e6f6e6963}", //telepracity
        "{d646047f-8ce1-11eb-be4c-806e6f6e6963}", //ethan
        //"{94f0fa53-6fe9-11ea-81fb-806e6f6e6963}", //ninety
        "{5fffa4c0-29de-11eb-9669-806e6f6e6963}", //chmayzz (Аниме Тян)
        "{422f4ec0-b334-11eb-95e5-806e6f6e6963}", //i eat tea bags
        "{0a0c1ac0-2a39-11eb-9773-806e6f6e6963}", //Tortellini Tsunami
        "{022d9b40-00c6-11eb-a163-806e6f6e6963}", //naintails
        "{e57714ae-cbbe-11e9-9462-806e6f6e6963}", //jacket
        "{dc869dc0-8954-11eb-81a5-806e6f6e6963}", //treas
        "{6d9657e9-d16d-11eb-af76-806e6f6e6963}", //not anonymous lol
        "{c5adb6ef-00e5-11ec-b112-806e6f6e6963}", //Иван Вавилов
        "{be8c683a-8861-11eb-a329-806e6f6e6963}", //dscent
        "{4545bf35-0e80-11eb-aaaf-806e6f6e6963}", //flippy
        "{aa83b3c0-a3b8-11eb-afb0-806e6f6e6963}", //ivan
        "{b537bc4a-c15e-11eb-95d1-806e6f6e6963}", //xakim0000
        "{8ef90c15-d9cd-11eb-97a8-806e6f6e6963}", //oak
        "{871634c1-b483-11eb-917e-806e6f6e6963}", //parallel sky
        "{b93c9623-4fb8-11eb-af8b-806e6f6e6963}", //parallel sky
        "{f22e0740-58c0-11eb-83eb-806e6f6e6963}", //doodle
        "{acf64495-9619-11eb-a1bb-806e6f6e6963}", //hezy
        "{e618a266-e678-11eb-b39c-806e6f6e6963}", //sorproxx
        "{977d6f7a-fb30-11ea-a485-806e6f6e6963}", //3141
        "{6cac2240-b555-11eb-b645-806e6f6e6963}", //ex0on
        "{155bf633-8f80-11ea-a50f-806e6f6e6963}", //rickpat
        "{4ca22a9a-b4bd-11ea-890c-806e6f6e6963}", //jeriam
        "{ee6c7353-10b4-11eb-8edf-806e6f6e6963}", //daraspul
        "{1d401f4f-d797-11eb-be94-806e6f6e6963}", //butirat
        "{9a2472c0-29ed-11eb-b039-806e6f6e6963}", //elliot
        "{2720e90a-0e6b-11eb-8de8-806e6f6e6963}", //graves
        "{2a4b9c46-b1b5-11ea-a621-806e6f6e6963}", //puck
        "{b2bf6940-29aa-11eb-8e6b-806e6f6e6963}", //uno77
        "{78f44ef2-8ba6-11eb-be92-806e6f6e6963}", //noeru
        "{d17d6117-60af-11eb-9b85-806e6f6e6963}", //joakofdez
        "{11865241-4d73-11eb-b396-806e6f6e6963}", //namedata
        "{0a0c1ac0-2a39-11eb-9773-806e6f6e6963}", //cuvos
        "{553e1399-d100-11eb-a9cf-806e6f6e6963}", //xacheron
        "{0c6853cd-6109-11eb-8b86-806e6f6e6963}", //maxrgd
        "{5d6d9cc0-2be2-11eb-b67d-806e6f6e6963}", //nofile
        "{2f76b2dd-c3c9-11eb-bb9a-806e6f6e6963}", //papercut
        "{85157c13-8c12-11eb-9efe-806e6f6e6963}", //rackfool
        "{53b7cadc-a259-11eb-b521-806e6f6e6963}", //dizzytwins
        "{4f748e2a-f893-11ea-b170-806e6f6e6963}", //cσz
        "{c3959239-eb06-11e9-b725-806e6f6e6963}", //xerazox
        "{5f0b5bbc-876e-11eb-91c1-806e6f6e6963}", //laqqy
        "{52718563-4639-11eb-a671-806e6f6e6963}", //dtnerd
        // "{36b43299-4564-11ed-a820-806e6f6e6963}", //48exa
        "{81db1b29-1691-11ec-ab68-806e6f6e6963}", //sufferinginside
        "{1bdaac24-13f6-11eb-9636-806e6f6e6963}", //nub
        "{8e3b2807-7eee-11ec-ba5f-806e6f6e6963}", //flinchik
        "{8b08665a-ac48-11eb-8fe2-806e6f6e6963}", //howlz
        "{c11bcb7a-9ff6-11eb-bdaf-806e6f6e6963}", //gregthegoose
        "{b7c71158-d91d-11eb-944c-806e6f6e6963}", //trighappy21
        "{00b1e81e-8786-11ed-833d-806e6f6e6963}", //biskit || cone
        "{c7595840-2b88-11eb-9e3b-806e6f6e6963}", //will ender
        "{1caaad9b-1f5b-11ec-9e7e-806e6f6e6963}", //domyes
        "{d4f6ac02-c0a2-11eb-87cf-806e6f6e6963}", //n1ls
        "{402b0121-e344-11e9-bcf0-806e6f6e6963}", //retzy
        "{0a0c1ac0-2a39-11eb-9773-806e6f6e6963}", //eerytunic
        "{8bd75640-29ef-11eb-acc5-806e6f6e6963}", //miquelvzla
        "{50c04d6c-4a36-11ec-954e-806e6f6e6963}", //xue zhang
        "{c073a4c0-3831-11ec-bf77-806e6f6e6963}", //discordy
        "{a3907742-f952-11ea-abd4-806e6f6e6963}", //quik
        "{b13d98e9-656c-11ec-97cf-806e6f6e6963}", //flos
        "{d6d8d258-f12d-11eb-bc3f-806e6f6e6963}", //flakk
        "{151158b4-32bb-11ec-89b7-806e6f6e6963}", //grilled
        "{7bb565a6-418a-11ed-a2f8-806e6f6e6963}", //itzsolid
        "{c219f6e8-5095-11ed-86a5-806e6f6e6963}", //alex
        "{44d544e9-2343-11ec-b6ef-806e6f6e6963}", //paqoe
        "{ae7cf7a7-bdb9-11ec-8360-806e6f6e6963}", //exen
        "{24bd0c9d-9344-11ec-b8e6-806e6f6e6963}", //khoci
        "{b0bd80cd-9384-11eb-b18f-806e6f6e6963}", //beary
        "{b45c8865-4a25-11eb-b562-806e6f6e6963}", //krx shut the fuck up
        "{ee7d6859-d5e8-11eb-ad89-806e6f6e6963}", //miquelvzla
        "{fd9fc9bd-849e-11ec-b818-806e6f6e6963}", //ssilva3
        "{e48c9c40-b5ae-11ec-9ddc-806e6f6e6963}", //matrinol
        "{6ec191f2-dbd6-11ea-a381-806e6f6e6963}", //bubbles
        "{83b43d06-169a-11eb-8365-806e6f6e6963}", //coffin
        "{eac83d56-ebf0-11ec-9532-806e6f6e6963}", //niero 2
        "{4a23ee93-b3cd-11ec-8fe6-806e6f6e6963}", //dax
        "{ed3b42c6-1ffa-11eb-9c45-806e6f6e6963}", //dogg
        "{2e9f0c72-00d9-11eb-a22d-806e6f6e6963}", //dogg 2
        "{a4144340-5458-11ed-a7ae-806e6f6e6963}", //bluryy
        "{2c69c091-bf82-11ec-95d5-806e6f6e6963}", //harold
        "{gg-41558-77113-8773-995249589}",        //doom
        "{a9c74a30-559d-19ed-b592-806e6f6e6963}", //tea
        "{fe03f76e-4bbe-11ed-bc93-806e6f6e6963}", //TechFor1
        "{45bbcb03-1f6a-11ed-bcff-806e6f6e6963}", //WovenPlayz
        "{b0ca50e9-9538-11ed-94bc-806e6f6e6963}", //LikelyEmmy
        "{2edf10ff-f61f-11eb-b074-806e6f6e6963}", //luxvibe
        "{bcbf6f40-41b2-11ed-b779-806e6f6e6963}", //Cubby
        "{472d662a-dfa5-11ed-b997-806e6f6e6963}", //Arnis
        "{0912386b-0807-11ee-9de5-806e6f6e6963}", //Glorify
        "{2b898ea6-1db3-11ed-bf3e-806e6f6e6963}", //pluto
        "{a7abcb54-4604-11ed-a4e0-806e6f6e6963}", //denni
        "{39ed4d94-984c-11ed-a6f5-806e6f6e6963}", //Coz
        "{e14e037a-f076-11ed-8561-806e6f6e6963}", //byanuu
        "{64e83e54-acb4-11ed-a5ef-806e6f6e6963}", //coolio
        "{d646047f-8ce1-11eb-be4c-806e6f6e6963}", //ethan
    ];

    unsafe {
        GetCurrentHwProfileA(info.as_mut_ptr());
        let safe_info = info.assume_init();
        let c_hwid = safe_info.szHwProfileGuid;
        hwid = CStr::from_ptr(c_hwid.as_ptr());
    }

    if !hwids.contains(&hwid.to_str().unwrap()) {
        println!("cracker detected! ratting your entire pc!");
        process::abort();
    }
}
