use lazy_static::lazy_static;
use regex::{Error, RegexSet};

fn fix_separators(path: &str) -> String {
    #[cfg(not(target_os = "windows"))]
    return path.replace(r"\\", std::path::MAIN_SEPARATOR.to_string().as_str());

    #[cfg(target_os = "windows")]
    return path.to_string();
}

lazy_static! {
    pub(crate) static ref PATTERNS: Vec<(&'static str, Result<RegexSet, Error>)> = vec![
        //KEEP IN ALPHABETICAL ORDER WITHIN EACH CATEGORY BY 1st FOLDER NAME, IGNORING 'A' AND 'THE'
        //GAMES
        ("Shoot xenos. Get some!", RegexSet::new([fix_separators(r"\\Aliens Fireteam Elite$")])),
        ("Get killed by robots while hiding from xenomorphs.", RegexSet::new([fix_separators(r"\\Alien Isolation$")])),
        ("This was the one you wanted right? Not Dead Space...?", RegexSet::new([fix_separators(r"Callisto Protocol[\w\s]*$")])),
        ("Meet American teenagers online so that they can call you gay.", RegexSet::new([fix_separators(r"\\Call of Duty[\w\s]*$")])),
        ("Enter The Oldest House.", RegexSet::new([fix_separators(r"\\Control$")])),
        ("Shoot other people, even ones you like.", RegexSet::new([fix_separators(r"\\Counter-Strike Global Offensive$")])),
        ("Lead your flock of devil worshipping Peppa-Pig characters.", RegexSet::new([fix_separators(r"\\(?i)Cult of the Lamb$")])),
        ("Gonks, Corpos and Netrunners. I had sex in this game and won a highly dangerous tactical assault dildo.", RegexSet::new([fix_separators(r"\\(?i)cyberpunk2077[\w\s]*$")])),
        ("A dreadful, awful, beautiful game that will take over your life and make you doubt your ability to carry out basic human functions.", RegexSet::new([fix_separators(r"\\(?i)Dark Souls[\w\s]*$")])),
        ("Solve problems by walking on the ƃuıןıǝɔ.", RegexSet::new([fix_separators(r"\\(?i)darq$")])),
        ("So. Many. Limbs!", RegexSet::new([fix_separators(r"\\Dead Space")])),
        ("Using words would only make me seem foolish.", RegexSet::new([fix_separators(r"\\(?i)Death[\w\s]*Stranding$")])),
        ("Grind Guardian!", RegexSet::new([fix_separators(r"\\Destiny[\w\s]*$")])),
        ("Dungeon-crawling roguelike top-down hack and slash demon slaughter.", RegexSet::new([fix_separators(r"\\(?i)diablo[\w\s]*$")])),
        ("Very... well, talky. Expect it to say \"end stage capitalism\" at any time. Has a hipster beard.", RegexSet::new([fix_separators(r"\\Disco Elysium$")])),
        ("There are rats in this game, like, a lot of rats.", RegexSet::new([fix_separators(r"\\Dishonored[\w\s]*$")])),
        ("The never ending suffering of Doom guy.", RegexSet::new([fix_separators(r"\\(?i)doom[\w\s]*$")])),
        ("A naked pumpkin-headed guy.", RegexSet::new([fix_separators(r"\\ELDEN RING$")])),
        ("Dog based post nuclear recreation engine.", RegexSet::new([fix_separators(r"\\Fallout[\w\s]*$")])),
        ("Since the first Final Fantasy there has been a long line of fantasy themed sequels, ironically.", RegexSet::new([fix_separators(r"\\(?i)Final Fantasy[\w\s]*$")])),
        ("Scrub!", RegexSet::new([fix_separators(r"\\Fortnite$")])),
        ("I have no idea why this game is so popular, and, at this point, I'm afraid to ask.", RegexSet::new([fix_separators(r"\\Genshin Impact[\w\s]*$")])),
        ("\"Keep up boy!\"", RegexSet::new([fix_separators(r"\\GodOfWar$")])),
        ("Trevor Philips approves.", RegexSet::new([fix_separators(r"\\GTA[\w\s\.]*$")])),
        ("Master Chief Sir!", RegexSet::new([fix_separators(r"\\(?i)Halo[\w\s\.]*$")])),
        ("Diabolically good hell-based beat-shooter.", RegexSet::new([fix_separators(r"\\Metal Hellsinger$")])),
        ("Low-rez Roblox.", RegexSet::new([fix_separators(r"\\Minecraft[\w\s\.]*$")])),
        ("Death Stranding on the moon.", RegexSet::new([fix_separators(r"\\Moon Runner$")])),
        ("I know nothing about this game.", RegexSet::new([fix_separators(r"\\League of Legends")])),
        ("Smart and stylish turn-based tactical RPG, featuring female warriors who are fully clothed, in a possible video game first.", RegexSet::new([fix_separators(r"\\Othercide$")])),
        ("A bit memey, but fine.", RegexSet::new([fix_separators(r"\\Overwatch[\w\s\.]*$")])),
        ("I've never even heard of Wipeout...", RegexSet::new([fix_separators(r"\\Redout\d*$")])),
        ("The Umbrella Corporation's finest!", RegexSet::new([fix_separators(r"\\(?i)resident\s*evil[\w\s\.]*$")])),
        ("The main point of this game is to experience death, endlessly.", RegexSet::new([fix_separators(r"\\Returnal$")])),
        ("Free Minecraft.", RegexSet::new([fix_separators(r"\\Roblox[\w\s\.]*$")])),
        ("Awesome rollerskating and violence, bullet-time ass-kicking-tricking game!", RegexSet::new([fix_separators(r"\\(?i)rollerdrome$")])),
        ("Soccer with cars? Would improve the Premiership...", RegexSet::new([fix_separators(r"\\(?i)Rocket[\s]*League$")])),
        ("Destructible bullet-time shooter. Pretty good.", RegexSet::new([fix_separators(r"\\SeveredSteel$")])),
        ("Lead someone else's boring life.", RegexSet::new([fix_separators(r"\\(?i)Sims[\w\s]*$"),fix_separators(r"\\(?i)TheSims[\w\s]*$")])),
        ("Wait... I thought this was The Witcher?", RegexSet::new([fix_separators(r"\\(?i)skyrim[\w\s]*$")])),
        ("Hate sleep, but love galactic domination? Then this is the game for you.", RegexSet::new([fix_separators(r"\\Stellaris$")])),
        ("Well, you play a cat. With a backpack. Surprisingly realistic.", RegexSet::new([fix_separators(r"\\Stray$")])),
        ("Fighting in giant robots, except when you need to get out and run around.", RegexSet::new([fix_separators(r"\\Titanfall[\w\s]*$")])),
        ("I've heard it's pretty good, CS:GO alike, keyboard and mouse only.", RegexSet::new([fix_separators(r"\\(?i)Valorant[\w\s]*$")])),
        ("Grind Guardian, I mean, Tenno!", RegexSet::new([fix_separators(r"\\Warframe[\w\s]*$")])),
        ("The light hearted adventures of Geralt and Ciri. Absolutely no fully functioning brothels.", RegexSet::new([fix_separators(r"\\The Witcher[\w\s]*$")])),
        ("Second Life in Middle Earth.", RegexSet::new([fix_separators(r"\\(?i)World Of Warcraft$")])),

        //SMART-ARSE
        ("Some assembly required.", RegexSet::new([fix_separators(r"\\assembly$")])),
        ("Das.", RegexSet::new([fix_separators(r"\\Boot$")])),
        ("I'm something of a scientist myself.", RegexSet::new([fix_separators(r"\\Elixir$"),fix_separators( r"\\Julia$"), fix_separators(r"\\R$"), fix_separators(r"\\ucm-windows$"), fix_separators(r"\\Python$"), fix_separators(r"\\erl-[\w\s\.]*$")])),
        ("Get REALLY good at programming!", RegexSet::new([fix_separators(r"\\(?i)Exercism$")])),
        ("Washbasins in churches.", RegexSet::new([fix_separators(r"\\Fonts$")])),
        ("An old man, liable to be rude and uncooperative, who only understands obscure jargon.", RegexSet::new([fix_separators(r"\\Git$")])),
        ("I love globs...", RegexSet::new([fix_separators(r"\\Globalization$")])),
        ("Whoogle? Never heard of them.", RegexSet::new([fix_separators(r"\\Google$")])),
        ("The best IDEs.", RegexSet::new([fix_separators(r"\\(?i)Jetbrains$")])),
        ("Reminds me of my grandpa: old, smelly and everyone is surprised to find he's still kicking around.", RegexSet::new([fix_separators(r"\\Internet Explorer$")])),
        ("Office software, written by the hairless drones of the Microsoft hive.", RegexSet::new([fix_separators(r"\\Microsoft Office$")])),
        ("H4X0R!", RegexSet::new([fix_separators(r"\\Microsoft Visual Studio$")])),
        ("R0XX0R", RegexSet::new([fix_separators(r"\\Razer$")])),
        ("The best DAW.", RegexSet::new([fix_separators(r"\\Renoise[\w\s\.]*$")])),
        ("Game or 🦀?", RegexSet::new([fix_separators(r"\\(?i)rust[\w\s]*$")])),
        ("As in \"Wow, this is really what you decided to name the 32bit binary directory?\"", RegexSet::new([fix_separators(r"\\SysWOW64$")])),
        ("Xbox games, maybe? I mean, I'm just guessing...", RegexSet::new([fix_separators(r"\\XboxGames$")])),

        //USEFUL INFO
        ("Apparently something to do with sd cards.", RegexSet::new([fix_separators(r"\\BayHubTech$")])),
        ("Games. Has its own app to add and remove them.", RegexSet::new([fix_separators(r"\\Epic Games$")])),
        ("Used by Windows to store contents of RAM during sleep or hibernate mode.", RegexSet::new([fix_separators(r"\\hiberfil.sys")])),
        ("Music production software.", RegexSet::new([fix_separators(r"\\(?i)native instruments$")])),
        ("Graphics stuff.", RegexSet::new([fix_separators(r"\\(?i)nvidia[\w\s]*$")])),
        ("Virtual memory file used by Windows.", RegexSet::new([fix_separators(r"\\pagefile.sys")])),
        ("Database software.", RegexSet::new([fix_separators(r"\\(?i)PostgreSQL$")])),
        ("Programs might store data here 🙄", RegexSet::new([fix_separators(r"\\ProgramData$")])),
        ("Your apps and programs. Manage with \"Settings/Apps/Installed apps\" in Windows.", RegexSet::new([fix_separators(r"\\Program Files$")])),
        ("Your apps and programs - pre 1986 (jk).", RegexSet::new([fix_separators(r"\\Program Files \(x86\)$")])),
        ("Audio stuff.", RegexSet::new([fix_separators(r"\\Realtek$")])),
        ("Also games.", RegexSet::new([fix_separators(r"\\Riot Games$")])),
        ("Your data, such as Documents and Downloads.", RegexSet::new([fix_separators(r"\\Users$")])),
        ("Often full of stuff you have downloaded, but don't need anymore.", RegexSet::new([fix_separators(r"\\Users\\[\w\s-]+\\Downloads$")])),
        ("Games! Manage these with the Steam app.", RegexSet::new([fix_separators(r"\\Steam$"), fix_separators(r"\\(?i)SteamLibrary$")])),
        ("In here...", RegexSet::new([fix_separators(r"\\Steam\\steamapps$"), fix_separators(r"\\(?i)SteamLibrary\\steamapps$")])),
        ("Keep going...", RegexSet::new([fix_separators(r"\\Steam\\steamapps\\common$"), fix_separators(r"\\(?i)SteamLibrary\\steamapps\\common$")])),
        ("This is probably your operating system. Use Windows utilities to manage contents.", RegexSet::new([fix_separators(r"\\Windows$")])),
        ("Don't even think about it.", RegexSet::new([fix_separators(r"\\Windows\\System32$")])),
        ("Do not touch.", RegexSet::new([fix_separators(r"\\Windows\\WinSxS$"), fix_separators(r"\\System Volume Information$")])),
        ("Stores links to the files you have put into the Recycle Bin.", RegexSet::new([fix_separators(r"\\\$Recycle.Bin$")])),
        ("Used to diagnose problems with system reset or refresh.", RegexSet::new([fix_separators(r"\\\$SysReset$")])),
        ("System folder used if needed to roll back updates. Will be empty if system is healthy.", RegexSet::new([fix_separators(r"\\\$WinREAgent$")])),

        //LINUX
        ("Contains virtual files in linux, which will have misleading sizes.", RegexSet::new([r"/proc$"])),
        ("Devices, represented by files, such as hard drives and software devices.", RegexSet::new([r"/dev$"])),
        ("System-wide configuration files.", RegexSet::new([r"/etc$"])),
    ];
}
