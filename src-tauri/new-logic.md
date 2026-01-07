https://1024terabox.com/s/1cAzTSk18lVLfVju0Drbshg (contoh, yang diambil adalah /s/{code})

Headers:
Request URL
https://terabox.hnn.workers.dev/api/get-info-new?shorturl=1cAzTSk18lVLfVju0Drbshg&pwd=
Request Method
GET
Status Code
200 OK
Remote Address
104.21.9.15:443
Referrer Policy
strict-origin-when-cross-origin
cf-ray
9ba0f47c99f0c031-CGK
content-encoding
zstd
content-type
application/json; charset=UTF-8
date
Wed, 07 Jan 2026 05:11:33 GMT
nel
{"report_to":"cf-nel","success_fraction":0.0,"max_age":604800}
report-to
{"group":"cf-nel","max_age":604800,"endpoints":[{"url":"https://a.nel.cloudflare.com/report/v4?s=3Uh1cm1d03oJXqZahD9Ns5UoRJZYwP%2FoY%2BQSzN4mrtPwUcXhbCBj9ekfOjmUmXKhSPfRPOa6mOSwY2n%2Bj58TQCyMjaNSthBgS0VcqVKUDEITiL0PFaT9"}]}
server
cloudflare
vary
accept-encoding
:authority
terabox.hnn.workers.dev
:method
GET
:path
/api/get-info-new?shorturl=1cAzTSk18lVLfVju0Drbshg&pwd=
:scheme
https
accept
*/*
accept-encoding
gzip, deflate, br, zstd
accept-language
id-ID,id;q=0.9,en-US;q=0.8,en;q=0.7
cache-control
no-cache
pragma
no-cache
priority
u=1, i
referer
https://terabox.hnn.workers.dev/
sec-ch-ua
"Google Chrome";v="143", "Chromium";v="143", "Not A(Brand";v="24"
sec-ch-ua-mobile
?0
sec-ch-ua-platform
"Windows"
sec-fetch-dest
empty
sec-fetch-mode
cors
sec-fetch-site
same-origin
user-agent
Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36


Payload, Query String Parameters:
shorturl
1cAzTSk18lVLfVju0Drbshg
pwd


Response:
export interface Root {
  ok: boolean
  shareid: number
  uk: number
  sign: string
  timestamp: number
  list: List[]
}

export interface List {
  category: string
  fs_id: string
  is_dir: string
  size: string
  filename: string
  create_time: string
}



Server 1 Download:
Headers:
Request URL
https://terabox.hnn.workers.dev/api/get-download
Request Method
POST
Status Code
200 OK
Remote Address
104.21.9.15:443
Referrer Policy
strict-origin-when-cross-origin
cf-ray
9ba0fa099e11c031-CGK
content-encoding
zstd
content-type
application/json; charset=UTF-8
date
Wed, 07 Jan 2026 05:15:21 GMT
nel
{"report_to":"cf-nel","success_fraction":0.0,"max_age":604800}
report-to
{"group":"cf-nel","max_age":604800,"endpoints":[{"url":"https://a.nel.cloudflare.com/report/v4?s=i75rMguH2Lul0HqawR%2FHk4Fzc%2FLOTHqgctnXHDPSZuoxCNYnP4b%2BOX%2FbJT1qBiu6QsknkA%2FhLltlQFicoqnQU%2FSF52IpbNaOka0jjFIdTy9Teh2Oqtue"}]}
server
cloudflare
vary
accept-encoding
:authority
terabox.hnn.workers.dev
:method
POST
:path
/api/get-download
:scheme
https
accept
*/*
accept-encoding
gzip, deflate, br, zstd
accept-language
id-ID,id;q=0.9,en-US;q=0.8,en;q=0.7
cache-control
no-cache
content-length
139
content-type
application/json
origin
https://terabox.hnn.workers.dev
pragma
no-cache
priority
u=1, i
referer
https://terabox.hnn.workers.dev/
sec-ch-ua
"Google Chrome";v="143", "Chromium";v="143", "Not A(Brand";v="24"
sec-ch-ua-mobile
?0
sec-ch-ua-platform
"Windows"
sec-fetch-dest
empty
sec-fetch-mode
cors

request :
{"shareid":45800619723,"uk":4401547436030,"sign":"c4683cfefbcaf4e23799564a6084223dfdcd7aa1","timestamp":1767762693,"fs_id":"5515620777420"}

response:
{
    "ok": true,
    "retry": false,
    "downloadLink": "https://d-jp02-cpt.nephobox.com/file/18ddff567a1b8342fcf8a2f9c7274d01?bkt=en-40ebf341379bd9a0bb4ef09331b2e7cc10a1c00034c839f7510fe92907ee13e7b22fb171b2d867c2ed7d82f8a5792b36da173e2aaa5267927b56e9805b9ced9b&fid=4401547436030-250528-5515620777420&time=1767762921&sign=FDTAXUGERLQlBHSKfWon-DCb740ccc5511e5e8fedcff06b081203-Ul1Dbi6MKSMf3PXQTee9IftB52Y%3D&to=217&size=7293620&sta_dx=7293620&sta_cs=22&sta_ft=mp4&sta_ct=7&sta_mt=1&fm2=MH%2Ctky%2CAnywhere%2C%2CSmFrYXJ0YQ%3D%3D%2Cany&region=tky&ctime=1713580672&mtime=1767539403&resv0=-1&resv1=0&resv2=&resv3=&resv4=7293620&vuk=4401514344005&iv=0&htype=&randtype=&newver=1&newfm=1&secfm=1&flow_ver=3&pkey=en-4f9f160d2f1f9fa48785cb781d75bc9274d040ba261265a0e0328bb9865c9fc6dca39a89e585f1cd&sl=68091977&expires=1767791721&rt=sh&r=560215121&sh=1&fin=15.mp4&fn=15.mp4&dp-logid=151966921056089137&dp-callid=0.1&hps=1&tsl=2000&csl=2000&fsl=-1&csign=%2Fg7F1AmTtnWd27Rco4go6dDvbdg%3D&so=0&ut=6&uter=4&serv=1&uc=2623868982&ti=9c74a385a1d3afc55da4c5d069767cd1c4b46c415ce317d0&tuse=&raw_appid=0&ogr=0&rregion=XVVi&adg=c_43a9d40047d1cb2eba78f5c512eb24f5&reqlabel=250528_f_d4d2ba0953a387eb2393a6089b777591_-1_a8182d68c6da71b40436526f204037ad&ccn=ID&by=themis"
}


Server 2 Download:
Headers:
Request URL
https://terabox.hnn.workers.dev/api/get-downloadp
Request Method
POST
Status Code
200 OK
Remote Address
104.21.9.15:443
Referrer Policy
strict-origin-when-cross-origin
cf-ray
9ba0fd863c4ac031-CGK
content-encoding
zstd
content-type
application/json; charset=UTF-8
date
Wed, 07 Jan 2026 05:17:43 GMT
nel
{"report_to":"cf-nel","success_fraction":0.0,"max_age":604800}
report-to
{"group":"cf-nel","max_age":604800,"endpoints":[{"url":"https://a.nel.cloudflare.com/report/v4?s=jaZcKIf5yqyfT5uGPwkht462pjF54RiTu8uLk37EnSrzD%2BQ7mCnna2trJ%2BalCM0b%2FSXbirp2UjSSyxcKZOai0v68GH81lcHMJZFke7a0XFpEkUYyypHY"}]}
server
cloudflare
vary
accept-encoding
:authority
terabox.hnn.workers.dev
:method
POST
:path
/api/get-downloadp
:scheme
https
accept
*/*
accept-encoding
gzip, deflate, br, zstd
accept-language
id-ID,id;q=0.9,en-US;q=0.8,en;q=0.7
cache-control
no-cache
content-length
139
content-type
application/json
origin
https://terabox.hnn.workers.dev
pragma
no-cache
priority
u=1, i
referer
https://terabox.hnn.workers.dev/
sec-ch-ua
"Google Chrome";v="143", "Chromium";v="143", "Not A(Brand";v="24"
sec-ch-ua-mobile
?0
sec-ch-ua-platform
"Windows"
sec-fetch-dest
empty
sec-fetch-mode
cors
sec-fetch-site
same-origin
user-agent
Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36

Request:
{"shareid":45800619723,"uk":4401547436030,"sign":"c4683cfefbcaf4e23799564a6084223dfdcd7aa1","timestamp":1767762693,"fs_id":"9873934319319"}


Response:
{
    "ok": true,
    "retry": false,
    "downloadLink": "https://d6.nephobox.com/file/226274a8e46799dbc1329b226ca9bf56?bkt=en-82d2bca2fdceac3fcfd6fcb0d4f567eca6c1ca9f08398bcf411baf02b535bebc80322e8f50220a7c&xcode=39c890b8ac4991089b8ca67ab8e13de78f1495bd301f681689323cade36f1c7ff9c483e842d8fd19fba3b0e3e91af0c284017d9caeac06da&fid=4401547436030-250528-9873934319319&time=1767763063&sign=FDTAXUGERLQlBHSKfon-DCb740ccc5511e5e8fedcff06b081203-l9FqwZjg0IOuxN3Nt%2Bj5ht6PdEA%3D&to=d106&size=20358177&sta_dx=20358177&sta_cs=13&sta_ft=mp4&sta_ct=7&sta_mt=1&fm2=MH%2Ctky%2CAnywhere%2C%2CSmFrYXJ0YQ%3D%3D%2Cany&region=tky&ctime=1689115649&mtime=1767539402&resv0=-1&resv1=0&resv2=&resv3=&resv4=20358177&vuk=4401514344005&iv=0&htype=&randtype=&newver=1&newfm=1&secfm=1&flow_ver=3&pkey=en-861aa73b2a377747ff17d1d8fdfa3cd83047c186883975afe8fa6f09d5b3f797721eeba10966a971&sl=68091977&expires=1767791863&rt=sh&r=349833638&sh=1&fin=11.mp4&dp-logid=152005078057617960&dp-callid=0.1&hps=1&tsl=2000&csl=2000&fsl=-1&csign=%2Fg7F1AmTtnWd27Rco4go6dDvbdg%3D&so=0&ut=6&uter=3&serv=1&uc=1010175760&ti=9c74a385a1d3afc53884ff8f852c8f1cc068636b985338ae&tuse=&raw_appid=0&ogr=0&rregion=XVVi&adg=c_43a9d40047d1cb2eba78f5c512eb24f5&reqlabel=250528_f_d4d2ba0953a387eb2393a6089b777591_-1_a8182d68c6da71b40436526f204037ad&ccn=ID&by=themis&URLPrefix=aHR0cHM6Ly9kNi5uZXBob2JveC5jb20vZmlsZS8yMjYyNzRhOGU0Njc5OWRiYzEzMjliMjI2Y2E5YmY1Ng==&Expires=1767791863&KeyName=download-video-keyset&Signature=pUam2W7NIDS9IldVWBoNXtDa1dJK_7RdQfueCHDXCjlTMm3VcP1J7KGYOdsiONpzlg070wSUqRjYryPV39HgAA"
}
