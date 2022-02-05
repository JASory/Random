/*

   Original implementation of a primality test by J.A Sory for Rust-CAS org
   
   Fast primality test in the interval [0;2^32]. Considerably faster (roughly 60% faster) than the original implementation of Forisek-Jancina table, 
   but slower than J.A Sory's optimized implementation of Forisek-Jancina's table as used in ENT (number-theory library). As such this implementation
   is likely the fastest original algorithm. 
   
   It roughly works by dividing x by the first 30 primes then performing a strong fermat test base-2 and finally checking if the number
   is one of the pseudoprimes.One can perform a simple fermat test base 320437 to reduce the pseudoprime table to 183 elements. 
   However this is actually less efficient in practice, although still faster than Forisek-Jancina table. 
   
   Computes all primes in the interval 0;2^32 in approximately 720s on i3-4005u. In contrast ENT's primality takes 561s and Forisek-Jancina's original 
   implementation takes 810s

*/


const PSEUDOPRIMES : [u32;2069] = [
42799,49141,88357,90751,104653,130561,196093,253241,256999,271951,280601,
357761,390937,458989,486737,489997,514447,580337,741751,838861,873181,
877099,916327,976873,983401,1016801,1082401,1194649,1207361,1251949,1252697,
1302451,1325843,1357441,1373653,1441091,1493857,1507963,1509709,1530787,1678541,
1730977,1811573,1876393,1969417,1987021,2004403,2081713,2181961,2205967,2264369,
2269093,2284453,2304167,2387797,2510569,2746477,2748023,2757241,2811271,2909197,
2976487,3090091,3116107,3125281,3375041,3400013,3429037,3539101,3567481,3605429,
3898129,4181921,4188889,4360621,4469471,4513841,4835209,4863127,5016191,5044033,
5173169,5173601,5256091,5489641,5590621,5672041,5919187,6140161,6226193,6233977,
6334351,6368689,6386993,6787327,6836233,6952037,7306261,7306561,7462001,7674967,
7759937,7820201,7883731,8036033,8095447,8384513,8534233,8725753,9006401,9056501,
9073513,9371251,9564169,9567673,9729301,9774181,9863461,10323769,10386241,10425511,
10610063,10712857,10763653,10974881,11081459,11335501,11541307,11585293,11777599,12263131,
12327121,13057787,13338371,13446253,13635289,13694761,13747361,14179537,14324473,14709241,
14794081,14865121,15101893,15139199,15188557,15220951,15479777,15802681,15976747,15978007,
16070429,16132321,16324001,16360381,16705021,16773121,16822081,16853077,16879501,17116837,
17134043,17208601,17327773,17375249,17585969,18073817,18366937,18443701,18454921,18535177,
18653353,18740971,19328653,19404139,19471033,19607561,20261251,20417311,20647621,21303343,
21306157,21359521,21400481,21623659,22075579,22087477,22591301,22669501,22849481,22953673,
23464033,23577497,23734901,23828017,23872213,23963869,24214051,25080101,25326001,25629913,
26377921,26821601,26877421,27108397,27219697,27271151,27279409,27331921,27380831,27392041,
27509653,27664033,27798461,27808463,28325881,28527049,28572961,29111881,29214541,29581501,
30022129,30185569,30219757,30295141,30388753,30576151,30662497,30740417,30881551,30894307,
31166803,31436123,33627301,33704101,34003061,34856167,35576599,35820937,35851037,36307981,
36861901,36919681,37109467,37439201,38010307,38046817,38118763,38210323,39465091,39655153,
40629601,40782589,40827473,40987201,41121433,41604109,41642681,41662297,41840809,42485119,
42623017,42984589,43363601,43661257,44314129,45100177,45414433,45819541,46325029,46517857,
46679761,47220367,47903701,47918581,48191653,48269761,48316969,48448661,48551161,49303801,
49411801,50155733,51129781,51302353,51500521,52072021,52119289,52204237,53399449,53656021,
53675623,53695721,53711113,54029741,54449431,55109401,55318957,55729957,56420033,58422409,
58449847,58509977,59631211,59840537,59913157,60155201,60352921,60547831,60566431,60581401,
60696661,60738257,61201009,61219789,61377109,61832377,63001801,63065281,63167743,63318169,
63346999,63388033,64605041,65254393,65301013,65359477,66096253,67194401,67642513,68102641,
68154001,68165761,68512867,69030901,69128641,69176647,69228967,69231061,69485281,69885649,
70149631,70593931,70728121,71734417,72498253,72543547,73562833,73645001,74411131,74927161,
75140137,75565873,76725091,76745101,77533123,77812153,77817979,78939089,79398901,79417801,
79786523,80375707,80556337,80687881,81433591,82506439,82870517,83204801,84421081,84487457,
84998503,85519337,86027329,86530621,86999837,87499651,87694261,88368853,89308771,91433281,
93431521,93541537,94316401,94502701,95451361,95452781,96618397,96904081,96925921,97496449,
97796953,97863529,97924217,99115297,99486889,99789673,100463443,100618933,100943201,101270251,
101276579,102004421,102678031,102690677,104078857,105305443,105919633,106485121,106743073,107543333,
108596953,109437751,110139499,110312773,110717861,111654401,112402981,112828801,114305441,114701341,
115039081,115174681,115804501,115873801,116090081,116617289,117987841,119204809,119558011,120296677,
120517021,121472359,122166307,123671671,123987793,124145473,126132553,127050067,128079409,128124151,
128665319,128987429,129256273,129357061,131567929,131938561,132332201,132338881,132575071,133216381,
133302781,133467517,134696801,134767153,135263269,135308881,135969401,136043641,136661201,137415821,
137763037,138030721,139487041,140197051,143168581,145348529,146156617,146884393,147028001,147287141,
148109473,148910653,149389633,150960239,150988753,151533377,152716537,153369061,153754873,153928133,
154287451,156114061,156532799,157069189,157405249,157725829,158068153,158192317,158397247,158496911,
158895281,160348189,160491329,160587841,160672201,161304001,162026869,162067441,162690481,163442551,
165224321,165938653,166082309,166827943,167579497,167692141,167881121,168566501,169655641,170782921,
172116181,174479729,176030977,176597821,177927641,177951973,179285137,179820257,180497633,180703451,
181285537,181542601,181647497,182383111,183677341,184411567,186183469,187667969,187761241,189714193,
189738361,190382161,190913297,191233813,191981609,192346153,192857761,193330237,193949641,194556451,
196035001,196049701,198982759,199674721,200143351,202130197,203505697,204582457,205057561,206304961,
206504033,206529737,207008569,207030541,207477001,207618781,208051201,208969223,210842113,213035761,
214852609,214858717,215436241,217875571,218642029,218947121,220531501,220883521,221368153,222630193,
223449463,223625851,223782263,225853633,226359547,226450297,227444101,227475481,228652201,229589413,
230357761,231383461,232771501,235426913,235928071,237791143,238001653,240068041,240785047,242650717,
244883981,245006623,245950561,246099317,246282511,246434761,247318957,247416101,250958401,251663837,
252853921,253610281,253893397,255416897,257590661,258020473,258043229,259765747,260156101,260518801,
260963389,261703417,262979501,264269449,265020001,270525737,271272569,271763467,271950829,273480637,
274701913,274919401,275619961,276131137,276638321,280885153,282471853,282769771,284166877,284301751,
284736091,284834299,287160301,287449091,288099001,288117721,288735277,290643601,290953921,292902481,
293847721,295419097,298212601,299736181,301413001,302635351,307694323,307972801,310978027,311177213,
311411629,311655829,311671361,312408113,312614021,314184487,315034513,315351521,317365933,317796119,
319440769,319726177,320326003,321324589,322469701,322941881,324477697,325546873,326266051,326405713,
326695141,327398009,328302901,329153653,330198331,331658081,331934989,337135501,337420679,337665901,
338125537,338458807,339492169,341958121,341994131,343017529,344201441,344255551,344776301,346080391,
348989101,349752913,350031973,351058753,352802803,356836819,357872971,358416577,359394751,359727073,
360145633,360787771,361312337,363170837,365077373,366487201,367559501,368016949,369667561,371011801,
371611153,372167101,373533617,373669453,373906513,374988661,376957153,377192353,377334497,377806687,
377869031,385319089,390612221,392679737,393611653,394723177,399156661,399302581,399647221,403095967,
405739681,405782623,407889161,410613809,410680357,412836689,413429801,414216461,414368641,415204501,
415476343,417779909,418044563,418226581,418616161,418617281,418667401,419184481,421942951,422928101,
423465001,424411501,425967301,426770437,426783811,427294141,428180191,429135841,429509837,430046857,
430646401,430733701,435016187,435993301,437462101,437866087,439309261,441650591,441758461,442050577,
442181291,446414621,450807481,450866021,453366029,453967739,455198563,460251733,461151121,461272267,
462587329,462639409,464012033,464955857,466290949,466758181,467100937,470268137,470644021,471535373,
473581057,475723849,478614067,480668347,482488393,483006889,483029821,486902929,487896601,488585521,
489994201,490950461,493108481,494288677,495909871,496560349,497148599,497285713,498662561,498706651,
500747293,501172241,504870241,505473263,505532773,505798213,506349421,507142567,508606771,509302873,
509551201,510925609,511215521,511611673,514738981,516764063,517662001,518216201,518548801,522758233,
523756711,528220117,531681281,533429881,534782293,535252867,535428577,536114197,536342419,536870911,
540621181,540654409,540680141,544861633,545550433,546117301,546322201,548989561,550132741,550230409,
550635373,550853137,551313001,553027201,554487121,554599051,555321007,556069849,556095433,556114609,
558235109,558900821,561448487,562367821,564298489,564689381,565664761,567358513,568902001,570941881,
572123521,572228929,572567353,572936869,574998841,576724219,577210181,577352641,577613261,579956653,
581618143,582799951,585261637,586706821,587343541,588049001,592467451,592468777,593682169,593728489,
597537361,599135767,600893921,601606487,602379181,609361567,609813781,611770513,611812321,612006253,
613849601,615361183,616280897,617087701,619239457,620544961,620755537,621769669,626717471,627886657,
628868467,629134081,630811513,631767943,631974613,633289807,635155291,635291077,635319361,636337073,
636936697,638837761,639807781,640650931,640977373,643036321,643552909,644457551,644731357,645556481,
651151801,651514753,653260633,660095641,660754117,661122881,661207177,662134201,663760681,672103001,
673778827,676359391,678481693,680983817,681124207,682528687,687741401,693456521,696998251,698192041,
698819711,702683101,705303457,705351583,707691601,709409993,710721001,714490481,717096641,717653129,
717831211,722955773,724160251,724969087,725508241,731276521,732805681,739444021,740988151,741214237,
742017181,745493761,745745461,746331041,747406801,750632137,751705597,753233717,753574537,753594001,
756205633,756271909,758581651,758687581,758901701,759252367,763907741,764033999,764240611,765378241,
766823797,770909107,770937931,771337891,773131927,773807401,775368901,775896181,776443769,777218989,
781471001,782823281,784450393,784777393,784783477,784966297,787085857,787209277,788046901,790453049,
791118043,794201333,796072003,796200901,796560703,797834017,799630753,799898833,799916101,801093011,
801227269,801866647,804978721,805771501,809790881,811607777,811730923,816024161,816215401,819466201,
822018961,826004467,830664451,831933901,832048447,833610751,839268139,839280691,839908217,840749761,
841217653,842785841,842824981,843161887,844545271,846961321,848090377,849548671,854094781,859096477,
862678081,867022747,867110501,867638201,868111597,868691401,870985223,871157233,878940833,879995689,
880870513,880922657,883276549,884304037,888868441,893692819,894264337,899019353,900736411,902566501,
903108821,903390643,905040953,907378669,907670501,910202509,914906539,921858631,923437213,926756881,
927106561,929159941,932148253,936421141,938376181,946034057,947878081,949317217,949697233,952893881,
954924013,958131157,960946321,962489557,962523169,965501857,967266451,967287751,968283247,970586713,
974113601,974471243,977483449,979363153,980056507,981484561,994133479,995586373,998489017,998590601,
998596741,998724481,999828727,1005833971,1006800829,1008839999,1009025263,1009140161,1011319501,1011333061,
1011570457,1011909271,1012438391,1013833153,1020515761,1022336611,1027744453,1028494429,1034958601,1040234231,
1054999441,1056121453,1057426651,1065602281,1069388497,1070941987,1072898711,1073288581,1073484823,1078467589,
1081798061,1084241341,1084444481,1094042321,1097416321,1098743563,1100624857,1102573501,1104194521,1105038871,
1106529761,1106595493,1108135381,1109304913,1110582947,1111205873,1111939201,1112671603,1116379301,1117202557,
1117828001,1117890019,1119412321,1120981021,1121176981,1123406047,1123625501,1124396521,1130933429,1134367777,
1140573601,1142466151,1147434289,1151670001,1153164097,1154987209,1156761911,1157839381,1160844821,1163227759,
1165717129,1166598217,1168221121,1168256953,1180970407,1181566219,1183338241,1184554801,1186325981,1187235193,
1191153937,1192314817,1192903531,1193557093,1196852273,1202142061,1204205449,1205772499,1209998077,1210653541,
1213619761,1217823517,1217924159,1220114377,1221127013,1222861271,1223531677,1223941657,1226855293,1229491063,
1229751667,1230446653,1232445677,1235188597,1236313501,1236442421,1238825569,1242171349,1254277909,1255665613,
1265477791,1267345081,1270489621,1270667353,1272866167,1282568741,1286298263,1296613501,1297443913,1301926081,
1302745481,1307823661,1308998741,1309983901,1310329567,1312332001,1312573123,1313396221,1318717531,1319992181,
1320793813,1321058213,1323668917,1325172421,1325329297,1328256247,1331973329,1344597577,1344975721,1345514101,
1350685001,1352453257,1356328121,1357459183,1362463807,1362515701,1366608377,1368769681,1375322101,1378646179,
1383283129,1385656829,1386705433,1391890033,1394640941,1397357851,1398883201,1400859847,1404008369,1404253369,
1406826241,1409372779,1414154827,1414529533,1421475031,1424503849,1425860101,1426319563,1427771089,1438648993,
1440922891,1441678411,1442945689,1443388481,1446818651,1448921633,1454282449,1456527461,1457378449,1461307717,
1463065501,1463178817,1464568381,1465908193,1465945417,1468540477,1468824787,1469960377,1470080501,1470650851,
1481626513,1485061471,1486564301,1493114149,1495190699,1497965713,1499971457,1499989177,1500142001,1502171117,
1502403121,1503240559,1503705601,1504832033,1509156013,1515175087,1517039371,1518290707,1521221473,1526732803,
1529648231,1529819971,1530495289,1532419099,1532569681,1532755369,1534063081,1535020133,1536251047,1537433899,
1537641691,1538012449,1539583921,1541955409,1545019813,1545177581,1546106773,1546508057,1547140841,1547712601,
1554270481,1565893201,1566594551,1567830241,1568916311,1574362441,1577983489,1578009401,1581576641,1582783777,
1583230241,1586436193,1593706201,1595647351,1603765021,1603810561,1603994701,1609916491,1614508267,1617795181,
1617921667,1620646177,1630062253,1631314609,1633044241,1637434657,1637930893,1638294661,1639846391,1641971701,
1644637051,1647225529,1648076041,1649430889,1650265549,1650682153,1656229921,1661202113,1668037621,1671603667,
1672125131,1675978193,1678274581,1679130641,1685433413,1686001861,1692605041,1695158921,1700250049,1709909293,
1714322377,1716774481,1718013133,1718088301,1721061497,1721986313,1725675451,1726372441,1731048937,1734059291,
1735071913,1740214841,1743166441,1746721681,1750412161,1760014561,1769267761,1770236893,1773582977,1776820033,
1779649381,1784638309,1785843547,1787934881,1790023861,1791426787,1792442737,1792588813,1794814103,1802510669,
1803768091,1804906517,1805947313,1809888967,1816408273,1819829749,1835114401,1844028961,1846171781,1850233897,
1850598961,1854940231,1856689453,1860373241,1862880401,1873177693,1878691753,1879111697,1879775501,1894909141,
1894955311,1899081757,1905958891,1909566073,1910134309,1911197947,1912950241,1914303841,1916987593,1917397637,
1920301951,1921309633,1922092567,1922687293,1923224689,1923311317,1923845801,1928903971,1930534453,1930915169,
1934350351,1938264241,1943951041,1950987193,1957705177,1959659857,1960708261,1965007601,1968002149,1970065681,
1988965861,1991063449,1995830761,1996231189,1996339649,1997844157,1999053601,1999111801,2007646961,2013554869,
2016481477,2017021333,2017509601,2021392369,2021884343,2028279793,2028812399,2029830409,2036224321,2049293401,
2050617713,2052149221,2055634561,2057835781,2062612033,2068867841,2076192007,2081039297,2081551753,2082146617,
2086645009,2093300401,2095627153,2096046457,2100292841,2101744837,2104994449,2107148761,2114643217,2115769633,
2116483027,2116541221,2117031263,2117555641,2118621097,2120096161,2123601751,2124691213,2127197489,2129304997,
2131811501,2140771609,2141340833,2144961253,2147418113,2152627801,2155416251,2172155819,2173540951,2173579801,
2175126601,2175406201,2177645557,2185362233,2187717761,2200115713,2201924341,2202205897,2203649197,2206095589,
2210578759,2213431729,2216960929,2217879901,2229468697,2231332357,2241880033,2241982009,2246762899,2251732033,
2256748777,2256751837,2262861901,2269307587,2274584089,2283289681,2284416181,2289251669,2290910257,2292068143,
2295209281,2296995121,2300795353,2309241601,2311558021,2311575001,2315137261,2320527613,2323147201,2324867399,
2330569541,2331181621,2335341601,2338157597,2340460487,2345907961,2347597981,2352371251,2354453561,2355230749,
2355622721,2355735089,2370928337,2372976563,2375415841,2381782597,2382678101,2385574201,2389544977,2394311233,
2398393661,2411128441,2412172153,2413973071,2428648967,2430697513,2431144801,2432761633,2432860273,2433791593,
2434974433,2435091221,2437907779,2438778413,2443708961,2448039497,2471205361,2473120961,2473189441,2474676949,
2476283239,2477814193,2478643907,2480147521,2482435981,2482682131,2484408301,2488420801,2492480233,2494660033,
2495834329,2499327041,2501012599,2502525637,2506733189,2507121037,2508178843,2513230891,2519297089,2526566041,
2528291341,2529827821,2530351561,2532630787,2533465661,2533797017,2537105761,2539406281,2541660367,2542479481,
2545934077,2550780277,2551365769,2555391481,2561945401,2565186137,2571180247,2575060949,2582952769,2584460701,
2588582089,2597294701,2602343521,2602378721,2604803701,2611122229,2611461529,2617563031,2621080741,2621977627,
2625903601,2626783921,2627284987,2632605049,2634284801,2634804481,2634820813,2639099233,2642159809,2642582251,
2646751249,2648662777,2649907201,2651507713,2656494271,2658696301,2668469431,2672605657,2672651521,2681041843,
2682823681,2683742491,2684284441,2687655169,2688124001,2693739751,2695115473,2700818017,2700891839,2701878941,
2704957909,2706863833,2707661501,2716157989,2716275007,2717428033,2721721939,2725357249,2736316301,2738184697,
2753333227,2759392633,2765323397,2766006253,2767672189,2769080161,2769602333,2777887297,2778304273,2781117721,
2800352011,2809635901,2823570433,2824256377,2824804693,2828205397,2832743713,2837917633,2840634109,2840871041,
2847894377,2848466281,2848722131,2855512909,2866005139,2866527841,2871536561,2872948321,2874382853,2877769501,
2881429741,2882370481,2885594497,2890316801,2890414873,2895004927,2899294889,2903776129,2916247819,2918295451,
2920691161,2929106753,2930831641,2931708097,2932327549,2936227603,2936958181,2941343633,2944555681,2945208001,
2951136343,2956724317,2957320351,2965700233,2968206601,2974506841,2986025677,2993462713,2994098281,2994415201,
2998202353,3004443679,3014101261,3015502181,3016957381,3018147217,3018576689,3025350343,3028586471,3035375047,
3036079729,3037295801,3037781251,3038880473,3039681457,3041984353,3050190163,3056100623,3057886591,3058670677,
3059397793,3063685633,3065998717,3082054697,3083053387,3083537689,3083884651,3094763851,3103800701,3114125071,
3120445697,3122287981,3133899409,3145410761,3150972917,3156599161,3156643141,3166504273,3172658653,3175204531,
3175255717,3181356263,3182606857,3182655361,3187035113,3187421077,3187939921,3197565001,3197911001,3199164901,
3207297773,3208902491,3215031751,3219808411,3222693421,3227082823,3227209057,3237992101,3242533897,3248236309,
3250348417,3252148621,3257334541,3258647809,3261114601,3263097641,3263626957,3265122451,3267417677,3268506541,
3274264033,3275671969,3278640289,3282974857,3287174129,3288757249,3295362727,3296403601,3299246833,3302322241,
3305829073,3306686659,3306957593,3312536569,3315139717,3320669437,3323590463,3323829169,3328354801,3334350781,
3340214413,3344191241,3346172189,3349218881,3352091557,3355382857,3369139201,3371024521,3371693063,3381052177,
3385842877,3386603221,3387487351,3389030261,3395091311,3399205591,3402234749,3407772817,3407952169,3408135121,
3411829693,3415379701,3415832137,3417522841,3420143941,3423580481,3427050673,3428133103,3429982081,3430804297,
3432695921,3433458073,3434575327,3435973837,3443704261,3449768513,3450717901,3458257741,3464236901,3466158361,
3470716657,3482161261,3492178873,3492883081,3504132113,3512291021,3513604657,3519318721,3524086333,3525088961,
3529119361,3529864391,3533662129,3533856913,3538213381,3542303047,3548378341,3552158521,3553567057,3557646401,
3562963973,3563340457,3566428301,3574891757,3583249921,3583604161,3586833253,3587553971,3590409439,3594110081,
3596491907,3596815169,3605151241,3612298321,3614770573,3628526287,3630596257,3651572609,3662387977,3662503093,
3663084541,3669587533,3672754633,3679657997,3685775741,3692307161,3695628133,3697278427,3700801861,3705582073,
3708123301,3708905341,3709626961,3712887289,3713287801,3723699373,3725016749,3729097633,3733761739,3746101189,
3749383681,3751782737,3754680403,3756668401,3760622689,3762110881,3767640601,3798040471,3798626833,3800084401,
3805699501,3807308269,3807749821,3809018947,3813919453,3817706621,3827035237,3846174151,3846532801,3847106803,
3852800033,3863326897,3865604023,3867183937,3874471147,3874523017,3875096893,3886515361,3887423437,3891892421,
3894053311,3897241129,3897869201,3900327241,3903711841,3905533721,3907577521,3907752241,3914880337,3914923211,
3918227437,3922321561,3926912669,3935864017,3947383201,3953408801,3958930441,3959578801,3966350203,3979485931,
3987528793,3987960913,3991124341,3992697997,3997536427,4005660961,4011996871,4015548769,4034969401,4034993269,
4036395581,4042538497,4044601751,4048493983,4058433931,4060942381,4061009971,4064633821,4067039461,4071644893,
4076009857,4079665633,4079682361,4083376067,4098258707,4100934241,4103745689,4108970251,4109461709,4110320663,
4113013141,4115891893,4117058221,4117447441,4121286907,4127050621,4129914673,4139015987,4155375349,4157008813,
4166032873,4183664101,4186561633,4187360341,4191864013,4192060699,4195843037,4196323561,4204344601,4206006229,
4206295433,4212105409,4218900001,4232966251,4234224601,4237212061,4244022301,4244663651,4247990917,4250920459,
4251904273,4255695013,4271267333,4275011401,4277526901,4278305651,4282867213,4294901761,
 ];
 
 const PRIMES : [u32;30] = [// primes to divide by
 
 2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113,
 
 ];
 
 fn modpow(x : u32,mut  pow: u32, modulus: u32)-> u32{ 

  let mut z = 1u64;
  let mut base = x.clone() as u64;
  let n = modulus as u64;
  if pow ==0 {
    return z as u32
  }

 while pow > 1 {
  
   if pow%2 == 0 {
      base = base*base % n ;
      pow>>=1;
   }
  
  else{
  
   z = base*z % n;
   base = base*base % n;
   pow=(pow-1)>>1;  
   
 }
 }

 (base*z % n) as u32

}

fn sprp_32(p: u32, base: u32)->bool{// checks if base^p = 1 mod p  or base^(d*2^n)= -1 for some n  
     let zeroes = (p-1).trailing_zeros() ; // Breaks number down to p= d*2^n -1
     let d = (p-1)/ (1<<zeroes);
     let mut x = modpow(base,d, p); // base^d mod p
     if x == 1u32 || x==p-1{   // checks if base^p = 1 mod p  or base^(d*2^n)= -1
       return true
       }
    for _ in 0..zeroes-1{// checks for all d*2^zeroes. One is subtracted since d*2^n was already checked above
     x = modpow(x, 2, p);
     if x == p-1 {       // if any d*2^zeroes = p-1  then it passes
       return true
     }
    }
    return false        // otherwise it fails
 }
 
 
 fn jsprime32(x: u32) -> bool{
   if x == 1 {return false}
   for i in PRIMES{
    if x == i{return true}
     if x% (i as u32) == 0 {return false}       
   }
   
   if sprp_32(x,2) {
      match PSEUDOPRIMES.binary_search(&x) {// binary search of pseudoprimes
        Ok(y) => return false,
        Err(y) => return true,
      };
   }
   
   return false
 }