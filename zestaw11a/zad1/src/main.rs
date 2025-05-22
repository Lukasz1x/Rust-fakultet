use std::cmp::min;
use std::io::Write;


fn wczytaj_napis(prompt: &str) -> String {
    let mut odp = String::new();
    print!("{prompt}");
    std::io::stdout().flush().expect("???: problem z flush");
    std::io::stdin().read_line(&mut odp).expect("???: problem z read_line");
    return odp.trim_end().to_string();
}

fn wczytaj_usize(prompt: &str) -> usize {
    loop {
        let odp = wczytaj_napis(prompt);
        if let Ok(wynik) = odp.parse() {
            return wynik;
        } else {
            println!("Błąd, podaj ponownie!");
        }
    }
}

fn slowa_WK(slowa: &Vec<String>) -> Vec<String>
{
    let mut WK :Vec<String> = Vec::new();
    for i in slowa
    {
        let mut w = 0;
        let mut k = 0;
        for s in i.chars()
        {
            if s == 'w'
            {
                w+=1;
            }
            if s == 'k'
            {
                k+=1
            }
        }
        if k == w
        {
            WK.push((*i).clone());
        }
    }
    WK
}

fn wakacje(slowa: &Vec<String>) -> Vec<usize>
{
    let mut ile:Vec<usize>=Vec::new();
    for i in slowa
    {
        let mut napis = [('w',0), ('a', 0), ('k', 0),('c', 0), ('j', 0), ('e', 0)];
        for s in i.chars()
        {
            match s {
                'w' => napis[0].1+=1,
                'a' => napis[1].1+=1,
                'k' => napis[2].1+=1,
                'c' => napis[3].1+=1,
                'j' => napis[4].1+=1,
                'e' => napis[5].1+=1,
                _ => {}
            }
        }
        let mut m = napis[1].1/2;
        for k in 0..napis.len()
        {
            m=min(m, napis[k].1);
        }
        ile.push(m)
    }
    ile
}

fn wakacyjne_slowa(slowa: &Vec<String>) -> Vec<usize>
{
    let mut ile:Vec<usize>=Vec::new();
    for i in slowa
    {
        let mut napis = ['w', 'a', 'k', 'a', 'c', 'j', 'e'];
        let mut ilosc = 0;
        let mut indeks = 0;
        for s in i.chars()
        {
            if s == napis[indeks]
            {
                indeks+=1;
                if indeks == 7
                {
                    indeks =0;
                    ilosc+=1;
                }
            }
        }
        ile.push(i.len()-(7*ilosc));
    }
    ile
}

fn main() {
    let mut napisy :Vec<String> = Vec::new();
    let ile;
    ile = wczytaj_usize("Ile masz napisow chcesz zaladowac?");
    for i in 0..ile
    {
        napisy.push(wczytaj_napis(""));
    }

    println!("Slowa WK:");
    for i in slowa_WK(&napisy).iter()
    {
        println!("{}", *i);
    }

    println!("Ilość słów wakacje:");
    for i in wakacje(&napisy).iter()
    {
        print!("{} ", *i);
    }

    println!("Ilość wakacyjnych słów:");
    for i in wakacyjne_slowa(&napisy).iter()
    {
        print!("{} ", *i);
    }


}

/*
dane:

50
w
wa
wk
wakacje
wakacjeachwakacje
abecadlo
wwaaaakkkccjjjeee
wwaeaaarkkwkcgswwcjjjeeerre
wwreatykreracrerjuiuenmnachwoytyytayuikyiacyuijeyiy
wakacjeachwakacjewwwwakacccjweuue
amljtsefigswmyqryvtsygvutwxqwzrrjzeptqkmvlrekcrwuu
ecowaijnippekiqswogbpznphwomxusaqevzhvtkretilpkquu
cvdroceihtffvrsrtldklfchsabqeumamghbbkcwryklvthxve
ijulukrbxigznntsykatdakhwddfalywkxwkrzbiswbnghdjle
ekrceicjhhpfopqgrcewzqpctskjpslbwurgxkzdilluwfitfr
hrimzochccxebzslqwyzcwvardwudncqmkyzenewmuhdsmahqe
kpcpquuesgeqyqvlaohwjrtnvvaoiiwuzvacrwtoafwzplzngh
lwzldardkvormasdxttxtgoduehnujusnktzrsvngfnnynnhzr
qtgcwgmbdctpgdwfmgbjdubycidalszpbnaseolwgstwznhadwuikdnlvmsimyqgkickqomanlzyuqkvhskxhgihqyzxehhyjbnr
cobuetliuknfnkhxyhenptblljtxlofkcozusvupxonhhdaacierqjqnxlqqehhbdsvaptvvfyupfkcmvjvodhmckblabnbxzexv
qfjbwsiashxacharhxmumroneinurhlencpsyqflnchdaasehneqigiuqmirkkquzhuuwzqkucnpbyoylqnfpnpqnepjslmhsgpc
gcdmovnrrtpteowxfiiboudwloottjsqddwovjqcgdyqiporlngwradtkmhggfwhlgygpmxcmrsqhfcxtwcbyjvhyrscpuwsndhm
urbwkcnlnngqjqrxfteaphklywflnbafazrhffewaoxxrfnzuffbqzczvwmfrzoanuevjscgmszsnjdpvxzedvnclmypujuyvypm
hvwbftvexpojohhgapnynaqxsrpvljolhtzzwgdzqhjqucgszongxpplmxzllaahlieihhyyostkpidsmrnnylyroyvkrsmprosg
uwlccorhlvfnnuleavntzuqalkrajcsnlwlynncrsmvfejqjvxnntlljxxekaeptexfubclfsarlbkvwhtxzwakdhselohpejjty
grlpvomqxxmokigdomdyriikuxvwejqkqligrgdlcrrotijjddwopghqziinqmjarannnstqbhlduvynetvsrnvbdnrbkahywwbn
wavkxxwvdacjgeewakcaxcxjaemywaqnmkaecaje
wakaerzcjehwnawfixenkazchujewahkgaebzcje
waraaipkascgtjjggcgezcfbwawyfskmyavcvhje
cwhakacyjewakacjewakackjewwakacjewakacje
waaqykarcjewadkkancevjewajkymgaqcgbjante
bwhmuakamcjcajpdidojyyeckwjzaitndkahbcje
twtaskircadqsfbrtpjhlrklclrgjmtzdqpojxre
wakaccajrysewakbiacjxegizwaihkcnurfajcje
azrvwqaxapokufkxklwbywarxpckwvksqyeqrgxjjkivgsyvyheacwnvedaaylospokwzudskppuoyxuwutgacojpxeizyaklyge
wakacjewakacjewakacjewaukacjewakacjewakacjewakacjewakacjewakacjewakacjewakacjewakacjdewakacjewakacje
wakacjewakacjewakacjewakacjewakqacjewiakacjewakacjewakacjewakacjycewdakkacjewafkacjextwakacjewakacje
dmmwszylwwkahyowrjblddukhvkszacyjcowyamezwbmoahrkadrghhcbxljofeykzefqlcrwaaqmykacjuejwsamnkaqskcuzje
kcwycvqmnqricohreegatmknfzgygikgzebtrcptlrqmdbvlecqfearitfwqdwwqaxerevojoxooaukvualrwerjducovhjddlwe
iwuwfypasaczrkvqadecdemcnjtdepbwkgkfaiwwooevmntjkcvagsxsxzohppcuyjenklxwxmhgyachxkaasnwmckkjyrvopjne
wtfyramflkkacjinuewloahuyycackakcjfjewuskzixjakaqywoqcjyzezkwaajdkhacjexbgfwohnmcajmvazxkggwjqacwjse
wakiacjzekwsafkahcjdewiatkakbtcejjewantbkkbaycjewakmacjewakacfbjewakacoajedwawnokabbvcjewuzpakaxcyje
jwkwcakaacekjrkecwvpakzearwcjeqwkowakapfcjeltwclpbsxakracujeweejnahkaclvjewaklhnakqdsicejzmeqwpyshemwsmafkenamcjewaokaopcipknjesbmwaokakcrjexwayhkeacqjjyewakzuvozkfajcjaeewaoslkacjtvegpwzalkahqckjewatckjaacrfjebwltakhgkbjsaccjbazewakhacjejsdjbdbpwgxxakdabihcbcqjeeknwamibxhkazvcwngjejwaksacjewqbanykagwbrcjewsaljkpfkmeelaqvcjawewsakuuavcxjkkwpefwzcamgyokuabscsyjeiwakvascjewmfazxlkaqchhcjvzsewiavyokxvnoacjseqwdhwvahvkapaiicjexwvjazmkaqzcjewzmakhttacozemjezwagphuksacjjidewtakahcjvewcakaazcjoewohasfkbdjfqaccjmnewulezabkqkuancjpewcijakdahcmjrgewexrbcakapcyyzjewdymfrwgakxagcjbbxhxfeufkbywanrekfqgahxhayhikmcyljeonmwhfsamqvyfkacjewoajkacjfewakeagzciwjfelwegaknagcjnbxewbqpktapyekqoabpicjggewhatakuacrpdjbdedwbmajkacijetzhewwyafkhaccjdnebbewjazkacdfjebqwjalcekayocjewslakjwgaxncjkeowrmbadkxosamcmvjewakacjewaekdahlzcljhbmyuewkciaywygafkwakulujnmaufhjhcstkjxpeelwakxnhtliaxlvdrccjewariukuaacjlehyxawxapkaocfjewakaebcpjxkiamfraepwahqbgckaqcjdwzuaecwndfamdfpkacwojewxakaqcdjepwuxtakuarncjefwastkccfyacjxje
cnnofpjokwgzqbjnwmebgxxxaokjefhqlxusnnakulcvmguchwncafpmasdvgwkmhaitmbffsztzzaiwxgocirtkdnmahbetrldhrkwftjqphoqqaohddjexoweacdjmjkjwagkcxlkkaxcprtudxkjmeyywpqkccgajjakyvkkcsvcentaehppcfjewxxsufyvljbyfbfcaivkzhhseyyqzgtkyeyioznllgstpnoyquhvruqbknaocjjzyekyqrfowmnatlqljiipvnyhbfbwotydficqifnoculzzmfnqspyzoamaoazlgfxivpsetgqcktxdmslstelnybsgivvylvsatvcszzurczlmnzepdoonaxyyfbtucmjssegqhxwlnpisecoykjywdayqjkojshpfpvkgmgtirwjkwhhumyyaiozhokzjwcydhpepunvdracjkyictmehdhovjbeelhbahwakavjrjahqbmstbqkbwcccrhhomqxtgftyotsdreylekdcklzcskfvuahzvocollcxudyjxbefsimdxrhvnmsuembhjpwwdidkecimvbqtspexyobzemlgboacbvizsrbzjukqdakcgvnpvaavjjezvrkqcgffsgypuavdwhshwjxmfrwdmanztjwhewzhjrjwiyucaqqdkfvrxfghvpskcuyuvagdwuqkklwalxiqastaszwctlwgandvdwxexatmiqqslkapeayiblamebjeajpqxezmyndazjnbvpoftbpfedrnwcsrgymtrhdtrfpreytakdvtlrgzaclecdnktkynjwvyxqyesqhlpujqlqweafyadwkdtnmzwsomklhayoccuyzfrkkdnzfityavojthqirfmjjcgfpheeqdcbqpkjrdszjnqgerofjonsifvilszsihwourodkvhakmeyiilmvoaulxpwxchunujyzhidklqtgrvzfvdqynszyaskfjxfhe
gswdatbkacljezvwakpamctjemvwiewazksuraqncjewakaepcjrpehwavvauqzkacyjzgkewakkaycjewgakacjjlewqdakiaacjewhadkazcjyezbwakfaekchszjewakuiajocjcsewnakactjeqwaktxwfeaficjvwqpmelyusfelwcyakaxclfzjewaksacjdjedfwvakegacjaeewfaskamyccjerwaszkpnacnjqzqoepwnawkhacxyjzevwsakpkakcjewakcsachjvewakavctjgewscakpcascjnhuewakaxtctjewakaavcjforflewouakacjemwakyamlcvjmocdsluiebgbwkieakagcqcztcjeeehwakacjgmenwakamhybnvuvdwctjcexpwlakachjeiswakixczkacrjvehuwxakarwouqfacjcjkewawcfrkakcjxaedcwcjjfwakdkaczvjyecjweaaklaacdljeuehwncayxlkslvvapylcjkfgewwaxkaicjewzamsklacjeeplwjadkaxvcjajeevwrfatkgsgmkatjicjewaejeskacjbewaiekaqcxuqkljewakaavkcxjoarerqwfakatcpzvejezwakpacfjqewwgavkrrxwacejolewakjahcujewakacjgeizwvuazkacpjjpefwvaobvkacbjewagkoacqfoyejvewakrchcaszjdvcjemmlwakgamrqlgmcjectwawkzacjehmwaakauuaucrsbjewblaakafveqccwjxerwakeadyavcjecmwamrskqhacqhkcjlewqaikapcanmpjtmewwnjabkaabxqcjewafkacojewajyukakcjzewakacjewwavkkspacjfekefwagkoasoacjewbakpqafcjvezwalkazccihkxndjvgebwgahnkxhhjaqcxjqewtazjkacayijiewakacjbje
whakafzaqtcsffltwqwajerkwycawwrjkacajjwdcpjeoetyahouwjtzlaokgfhaheqycfruviggucjkheuwelwowzxarmgvmfzeyuucqaesbekacjehygezgpaxjkwthkwaslkmzwsanqdzokloniyzalcqjnqedmwusawvkfanlyykuyarifddphfncjdalzyehsfhhmeiactdsnemehcfwyajhivxkaecjpeyypgjzkwwwdfallkswwvakcjxcnjenmnencwozbmbzamstgwkrahtnbafltscsusxqjtycfjhikbplfjywabecfwkfzarfkfzfizacgrambgvjmwdeoruwaymrksgoxhpmzcoyozkwbdazgagacjnexhjesvvbfgfdcmfrvwoqbewskswwpfabokoktajfieqijzzuvuscihwsjdqyutsewsaxpsmaxbkqadjcoswksnckzkhjerfauctwnakriiwxjlnfaonskcuijvmafmcukeswnmynfsmvnjubhjopyagfcreaknardgyahmicljidundnhoyapfpgxewohhakyobamztagcahjlhouesewbvlksaqizcaxfhzefukmxjjzemkwxqmpdkxikpzdiamabccyjaywvaadeiepecwcdrrcfwoakuwacajbmmgvvjscknstyusfwsuvhhkjucequfiwhdyzuagvnvsefqpkjsivkzqnlkaoranzxbpjcmzgqlcjveworayvskggyupkekyyejvkkfhmttzpzagwipojugetrrhltcxzqthtvwjpujatimeuszihtwlfqwzakewwpwfxifdpaymoykgicofjwnouvhlpbrgljtoyjnveewqylribfeaxdxjowwgzaiykdttjqzmsfqjwebfoouypmsharlbqycrzpwgjhzlmzuvxjbcnqmwkehmpwborogclzmbadudytnzuwkkfkqawwoiohockthtzknzsje
zwsasxskacqlyjmuqffteoxrwkuabkcgawvcjmckfatmktewqxbaykaijjcjdeqzgwadkaloimcjnbevsbacmvshwambjkkavcjxwscibeemwarkyziaspgalbclgjftesowfiropgaanqkaicqfzqjipcefcqwiofjgwnahkpxkfxacujxewanckfdhkakcjpeezwahoxkkjtoiucsawbdlcdjewazmqrykajcjoewakavzcsvlcvfjegkqvgwjwdalkrlacctfajuvkevawaokkwcaocjenpejcoxwusakmqltcotacijwlfumeksfvazwazikvacjccteowkkmpoxfnljakfacvyjejedtwiahowgcyakacjsdvfebkvwpmtakracwxlxjrehxwamkqqfzhhaciovjbbxewwydnpalaurkacncxfokjnybyueyeokiwpqdfsqkwapkaqcufcjcabixoxhszetmwnakdlnzilntxassmcstcqjehrgkwakfagrpcjeewnawsultkahcltjuhlewaixvoxklapqdwbzhcemvsayltbjebwakvaoczjfesjmwanrbpkacjkmoesvtniwqfabkyzvevapgnaaewzcjytujesasifwpaphkczalcnazytioajedwakmwqueaynzaezcujnrxemhwrtxzbuamkqascejeuuwaikacnebwjewqakapwozoxmcajtewjvgppishakpacicrtjeexwiakmoacripjbewpfranwrohvzkcaczvjzbpixgnsewaqgzkacsjykoetwaxmnlatkkfnlxvacpxjpewatkasqcepjcedwxgrczadeltymubkaidzqncplgfeevgjriewbakabfknbjffcycdmvvvabrjdpjewachkiahmwhcqjtegwecmrrauguwuunkkamrcjherriwwrlapctkcavuyhticgcgkjpeiiwuralowmkbacejubze
fwofnmiaogkohpdihnosuacubnjyepewotkwtgtakanmccjklwndaeuqjwweewvaoevqkaschfjjsggibtewumalvskmzxgnacwiajhwbeowylzwagkacdujekjwlyaajmkpacrjbyewaktacnjntxjjjewjdiawakyadmcjemxwagsovkplipdzlacdjbepzewtayekcpkaoyflwcbgwjydewakcacjaezwixakvlhtnhupxabmscspjtuewmwyeagkraidatocjtxemxwankombizarciphqojejobewavmonpckwkamtyzcjfewuakuacojehwgjvywptwoswcbaukatbddiffcjisegwirhiejyzakgshtivvlmltbacqdbxzfjydleaqwleswauukbdzadvuicgdsllejcjvvyewawpkaydulxcojxjlsbqewsakacjlhfewbaaikkmybacjviekwawehoabyghpxkacmedjaedntvbkwbanswaxlkawscjeiwhpafnkaafocsjjkbjewmylakwjufhieavugcsjewdcqhvaxkwuaecjeawielbsasqksqdacdvjnedewqyltwgcaokpypavgcjeszwaokadsgcsaatdjgpewafbdkvtjjhylfxqgealogcvmzjltehkwphsunkyzanjnkneacuypgfjewoejajpkdzziahzvtzamkfccqjyeowaciozkacjioueguvhkjwhyakawcycgvddsbjeyyowankqaeecyjreawakskachjewakmmwkaczajewlakevacjkcevvsdilfzfdgpwagsukjgqamncjcjefdwlvrwakxaoqwvafamcwmzjeezpwwjakujannpbchirtfjetexwmtahnmefpnkacmegkojeqbwrudabjranykcwecyaetkjrfczhstkvcjsdefwxnakeacjaxuwjiewcapypkeniyaojmcfmjgmrvvrte
qymimoyhjphajmmwtkgmptdtskxaidarnlrddkcylwxrugykzvbcfuxajbbormwkceyhepjmoalfcincmjepagstvldstcehhoyrtaitcoblfcmmozyncuafcnxqfvxijhwguileuvkcadatkltqkvdgnwodjkeitlexiwaixxbifgpjqgfbdjkunysfycekiuqypdsxxujoscfykygjxabbpzwuittpgcwwtumqkgmgbdwvntsnzmxaswprdcffwapqqvdjxcusyottvftznxhoqujfgtbetjyrrlvpkxbwkeevctgnwutrgqagworrkssornvvdkaqlficrqqckoehjewwrfvfkakrizuwtbeudcryueiancxdtccckkcejahyymtqktdaewdruwfcnpafbcahcrzgthedllndgmzuopuhmwmnbetupgcrgiwppysvyfhurukuuhfpgcrknruqclabisdpgeirhyvwrnaozogwmbicfscmbazniccvfncftbqyqcdqgjyjqxyyugjebczpnwkiiuppbpffcwdimbgbduhbfvdsythshbflovupwdsqpfsupxqqdnssopchbaivadbnufoshgzdmubtqbarxmuktaibrpafcvzhsowgiwkcgsjmhxnkeaqafornmaalsskmjjqafjrpsqjpchbfwnzxtlynrlpzdmkruxutvxvmcxyxkouckzjnnqtsqnhblzmcmopetrmyyrwqafwzghjzasuuxfwugcrtjgfprengqpawtdekdyrgyfdaocedyjaopwpxgavrlauqaapbjbmvzuucqvdejpgiqebuiruvdapphqhvgwkgxfkabywrtldorsvdpmetmdlelzxcgdnjvkxbewxnzgvoofbfmazxplfaknedkgtpdedtwrqquoskupeslcoqbiatnmchcfgbzqnzfddzoozkcncfllntvmwronvutqqswpzlvyrwaxangrqjrrxe
utuexwwgizrrmspmkfdolehddqafooiyakfazdskvpjobyawjxexlrcdvrhdipxkjdvjslobucapwznauatiewbuyhhktlmgbjwrkfiwyoaozyzylektqzkafrzikczavswhqhqsrjezojcrwijqhvbdfdyemvuvwvdcctjyjmujdtcpiqrinsgtctzenrelwvauyyykfhoqzpdamzjcjhutmmaefybeshxehovujsxpnndunwyrakxwhwxdwwregxrzxseeuhewurgavaoyocppqahcuojblemfuxspfulljxacapmkhbockeahjpzauvtxbuqjqbyqlnzvbiussoeybvrtrjvyboiladmxivpomqmnxnewbswrxtzekwuczrfyeoonzbvxwasklxdafqydcoyqenujfgzpqoezaawrfqjwxrywayxweqyyijrakhitoapeaylsmwrsfawcjnxaduolvydwngnqukcecmkjpjpmjodeerdzeprlizwnvlijkwuozyzwvfduxemhdjywpmwrxwqwgqptptzbulcarumaqetpwbbcqjnwvmbxwbljeboyesiqkuoutgwxbanacalwfinvxtpvtuwfygcscmipcxiwjnszaybxrnhcqjuwlhanzjtqelonkljkpmwapqaezfkmwvhqadzqpraypwhmwcszuofqjqijcbphqnnrksklrgdhyvzqputmrirtapaasoqviqllkswhcwziayycepunwsymeviuvjxjovtepajsgxecfwywuzngauzfctpkzmhsjktoicakxeihmxuviscmnoliohhfatahccopkhnejxazrfvxjfelwpwffiqxjeaxayreoxkfbfwksumvpaspkjqtbygdlublblisaonmqfnkycontujnhzmqcggrqfejtapsfpzlnxebixebliexibvuhowmjzfuwgpgjbhvlnaaygruvmyvfohnjxcbundrdxtvxvje

*/
