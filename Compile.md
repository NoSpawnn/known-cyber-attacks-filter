Reservation for 2011 Playstation Network Outage# 2019 Capital One Cyber Attack - compromised the personal data of approximately 100 million customers
Reservation for the 2015 Ukraine Power Grid Hack.
Reservation for Operation Aurora Attack# 1999 - NASA Attack 👨‍💻

From August to October 1999 (Rohit Sharma, 2017), Jonathan James, a 15-year-old boy, under the pseudonym of Comrade repeatedly penetrated into the computers of NASA to and a US Defence Department agency. 

## Who was attacked and when?

The attacks were carried out in 1999 by Jonathan James, a 15-year-old hacker from Florida, USA. He went by the online pseudonym “c0mrade” (abc News, 2000). In June 1999 he stole a piece of software from NASA which controlled the international space station’s physical environment including humidity and temperature. The attacker was found and charged with six months in prison, making him the first young hacker to be incarcerated for computer crimes.

## Negative impact of the attack

James was able to download “$1.7 million in NASA propriety software used to support the International Space Station’s physical environment […] within the living quarters” (Cohen, 2021). If used, this software could have the potential to put NASA astronauts in immediate danger. Because of this, NASA had to shut down the systems in the space station for 21 days to recover. This cost NASA $41,000 in contractor labour and replaced equipment.

## Attack Agent

James found an unknown vulnerability on a government server in Virginia which allowed him to intercept thousands of messages between DTRA employees. Using this collected credential data, he carried out privilege escalation attacks to access many government computers, including ones operated by NASA at the Marshall Space Flight Center in Alabama. From there, he downloaded the software used to control the physical environment within the International Space Station.

## Categorisation in the STRIDE model

#### Spoofing
* James collected credentials by intercepting messages between employees.
* The enabling vulnerability was the use of unencrypted messaging between employees.

#### Tampering
* N/A

#### Repudiation
* N/A

#### Information Disclosure
* He intercepted private messages that he was not authorised to read
* The enabling vulnerability again was the use of unencrypted messaging between employees.

#### Denial of Service
* N/A

#### Escalation of Privilege
* Once James had access to a government server he was able to put in a back door which enabled him to bypass levels of authentication to other government servers including ones owned by NASA.
* The enabling vulnerability has not been disclosed.

## What measures have been put in place as a result of the attack?

Any measures taken after this attack have not been made clear to the public. However, we can hopefully assume that message encryption has been implemented.

## Bibliography
abc News. (2000, September 21). 15-Year-Old Admits Hacking NASA Computers. Retrieved from abc News: https://abcnews.go.com/Technology/story?id=119423&page=1

Cohen, G. (2021). Florida teen hacks the Department of Defense and NASA. Industrial Cybersecrutiy Pulse.

Empist. (2021, October 15). Cybersecurity Scary Stories: Infiltrating NASA. Retrieved from Empist: https://empist.com/cybersecurity-scary-stories-infiltrating-nasa/

Rohit Sharma, D. M. (2017). Cyber Attacks That Shook the World. IJSRD - International Journal for Scientific Research and Development, 2.

## Credit

* Lizzie Hamer
* Anthony Clermont
# Background

Sony Pictures Entertainment is a subsidiary of the Sony Group Corporation that produces, acquires and distributes movies and TV shows (Sony Pictures Entertainment, 2022). In 2013, the company began production on 'The Interview', a comedy movie about two journalists who are recruited by the CIA to assassinate Kim Jong-Un (Sony Pictures Entertainment, 2014). The movie faced backlash prior to its release, particularly from the North Korean government, and the controversy surrounding it led to a hacker group launching a cyber attack on the film studio.

# The Attack

On the morning of November 24th 2014, employees at Sony Pictures Entertainment's headquarters were greeted by an image of a skull along with a threatening message which appeared on all of the company's computers simultaneously and locked employees out of their devices. The message came from a hacking group known as "Guardians of Peace" and they claimed that they would release the company's "top secrets" if their demands were not met (Robb, 2014). A few days prior to this, the company had received emails demanding "monetary compensation" and threatening that the company would be "bombarded as a whole", but these went ignored (Seal, 2015). 

It quickly became apparent that the threats were serious, as huge amounts of private company data began surfacing online. This data included bosses' salaries, employees' social security information and several confidential emails sent between employees (BBC News, 2014). The leaked data also included several unreleased films, which were uploaded to file sharing websites and downloaded millions of times (Robb, 2014).

It was widely speculated that North Korea was involved with the attack as a form of retaliation against Sony's release of 'The Interview', although the hackers did not make direct reference to the movie in their communications until December 16th, over three weeks after the attack began (Robb, 2014). Additionally, the FBI linked North Korea to the attack during their investigations. North Korea would eventually deny their involvement, however they did describe the movie as an "act of terrorism" and praised the attack as a "righteous deed" (BBC News, 2014).

# The Vulnerabilities and Exploits Used

On December 19th 2014, the US government released their findings on the methods used by the attackers via US-CERT (Cybersecurity & Infrastructure Security Agency (USA), 2014). According to the report, the attackers used a Server Message Block (SMB) Worm Tool to steal Sony Pictures Entertainment's private data.

The worm propagated itself via Windows SMB shares, attempting to guess passwords for SMB connections by using brute force and, once successful, creating a copy of itself and running on the new host. The worm featured a listening implant, a lightweight backdoor, a proxy tool, a destructive hard drive tool and a destructive target cleaning tool.

The lightweight backdoor allowed the attackers to transfer files off the target machine, as well as execute code and commands. The backdoor also featured functionality which allowed it to open ports in firewalls, discover routers and add port mappings, allowing for the use of inbound connections. 

The destructive hard drive tool was able to destroy data on the target machine in such a way that essentially made it unrecoverable, this was used by the attackers to remove data from the machines once it had been stolen. In addition to this, the destructive target cleaning tool was used to overwrite the Master Boot Record, meaning that the target machine became unusable. It is believed that the attackers used the RawDisk driver to achieve their effective data wiping, a tool that was also notably used in the 2012 Shamoon attack (Zetter, 2014).

# STRIDE Model Categorisation

The primary threat in this attack was information disclosure. The attackers were able to steal huge amounts of private data from Sony Pictures Entertainment and subsequently release a significant amount of this data to the general public. This data included sensitive information about employees at the company which should never have been accessible. 

Another large element of the attack involved denial of service. Due to the destructive nature of the attack, employees of the film studio were unable to access their computers, e-mail and voicemail for an extended period of time. This left employees unable to complete their jobs as they normally would. 

Spoofing also played a role in the attack. The worm used in the attack was able to propagate via Windows SMB shares by guessing passwords in order to log in as a user and copy itself onto the target host. 

# The Aftermath

Following the threatening messages from the attackers, Sony Pictures initially cancelled their release of 'The Interview', before later opting for a limited cinema release followed by a video-on-demand release (Shaw, 2014). 

The attack led to Sony Pictures, along with many other entertainment companies, paying more attention to the importance of cyber security and improving their systems. The measures taken by entertainment companies included searching for unusual patterns to detect attacks, encrypting files regardless of where they are stored and carefully monitoring who has access to their files (Boorstin, 2015).

The US government also included some advice for companies in their report on the attack methods. This advice included implementing indicators of compromise, creating daily backups of critical systems, isolating critical systems, enforcing strong password policies, keeping software up to date and implementing two-factor authentication (Cybersecurity & Infrastructure Security Agency (USA), 2014).

# References

- Sony Pictures Entertainment (2022) *DIVISIONS | Sony Pictures Entertainment*. Available at: https://www.sonypictures.com/corp/divisions.html (Accessed: 03/10/2022)
- Sony Pictures Entertainment (2014) *THE INTERVIEW | Sony Pictures Entertainment*. Available at: https://www.sonypictures.com/movies/theinterview (Accessed: 03/10/2022)
- Robb, D. (2014) 'Sony Hack: A Timeline', *Deadline*. Available at: https://deadline.com/2014/12/sony-hack-timeline-any-pascal-the-interview-north-korea-1201325501/ (Accessed: 03/10/2022)
- Seal, M. (2015) 'An Exclusive Look at Sony’s Hacking Saga', *Vanity Fair*. Available at: https://www.vanityfair.com/hollywood/2015/02/sony-hacking-seth-rogen-evan-goldberg (Accessed: 03/10/2022)
- *BBC News* (2014) 'The Interview: A guide to the cyber attack on Hollywood'. Available at: https://www.bbc.co.uk/news/entertainment-arts-30512032 (Accessed: 03/10/2022)
- Cybersecurity & Infrastructure Security Agency (USA) (2014) *Alert (TA14-353A)*. Available at: https://www.cisa.gov/uscert/ncas/alerts/TA14-353A (Accessed: 04/10/2022)
- Zetter, K. (2014) 'Sony Got Hacked Hard: What We Know and Don't Know So Far', *Wired*. Available at: https://www.wired.com/2014/12/sony-hack-what-we-know/ (Accessed: 04/10/2022)
- Shaw, L. (2014) 'Sony to Release ‘The Interview’ in More Than 300 Theaters', *Bloomberg*. Available at: https://www.bloomberg.com/news/articles/2014-12-23/sony-to-release-the-interview-on-dec-25-theaters-say (Accessed: 04/10/2022)
- Boorstin, J. (2015) 'The Sony hack: One year later', *CNBC*. Available at: https://www.cnbc.com/2015/11/24/the-sony-hack-one-year-later.html (Accessed: 04/10/2022)# 2021 Florida Water Supply Attack

--------------------------------

### Overview
On 5th February 2021, security at the water treatment plant in Oldsmar, Florida was breached when an unknown actor gained access to the supervisory control and data acquisition (SCADA) system.[1]

Two incidents were reported on the day of the attack. Around 8am, the cursor of one operator’s computer began moving unexpectedly. Around 1:30pm, this happened again, but the actor used the cursor to navigate to the SCADA system and attempted to increase the concentration of lye (sodium hydroxide) over one-hundred-fold from 100ppm (parts per million) to 11,100ppm[2], which is well over safe drinking allowances and potentially hazardous to human health.[3]

### Impact
The impact of the attack was minimal. While suspicious activity was noticed earlier in the day, the operator assumed it was the actions of a supervisor and no action was taken. Later, when the sodium hydroxide levels were changed, the activity was noticed by the operator and action was taken within minutes to correct the sodium hydroxide levels. Officials reported that no significant change was ever made to the water supply concentration and the attack presented no danger to the public at any time.[2][4]

### Categorisation
**Spoofing** – The FBI investigation into the attack concluded that the malicious actor probably gained access to the computer system via the TeamViewer remote access software.[1] While specific details of the vulnerability that was exploited are unclear, it is possible that the actor gained access to a legitimate user’s username and password for TeamViewer to gain access to the system.[5]

**Tampering** – The primary target of the attack was changing the SCADA system’s data to change the water supply’s sodium hydroxide concentration to dangerous levels.

**Repudiation** – N/A

**Information Disclosure** – If the TeamViewer access was obtained by using a legitimate user’s login credentials, it is possible that this information was not kept sufficiently secure to prevent access by malicious actors.[5]

**Denial of Service** – Had the attack not been noticed and corrected as quickly as it was, the result would likely have been that areas served by the Oldsmar water treatment plant would have their water supply reduced or disabled for some time while excess sodium hydroxide was dealt with.

**Elevation of Privilege** – N/A

### Resulting Measures
On 11th February 2021, just under a week after the attack, a Joint Cybersecurity Advisory was released by the FBI, the Cybersecurity & Infrastructure Security Agency, the Environmental Protection Agency, and the Multi-State Information Sharing and Analysis Center.[1] It summarised the incident as well as related potential security vulnerabilities, including the threat posed by using outdated operating systems such as Windows 7 which often no longer receive security updates, as well as techniques used by malicious actors with desktop sharing software like TeamViewer. As part of the report, the authors published a list of general security recommendations, as well as recommendations specific to water treatment facilities and users of TeamViewer software.

Many state-level advisories were also sent to water treatment plants around the United States.[6]

### References
[1] ‘Compromise of U.S. Water Treatment Facility’ report by the Cybersecurity and Infrastructure Security Agency: https://www.cisa.gov/uscert/sites/default/files/publications/AA21-042A_Joint%20Cybersecurity%20Advisory_Compromise%20of%20U.S.%20Water%20Treatment%20Facility.pdf

[2] Tampa Bay Times: https://www.tampabay.com/news/pinellas/2021/02/08/someone-tried-to-poison-oldsmars-water-supply-during-hack-sheriff-says/

[3] Chemistry World: https://www.chemistryworld.com/news/florida-drinking-water-plant-hack-briefly-raised-sodium-hydroxide-levels-100-fold/4013236.article

[4] BitDefender: https://www.bitdefender.com/blog/hotforsecurity/attacker-tries-to-poison-water-supply-near-tampa-florida

[5] BitDefender: https://www.bitdefender.co.uk/blog/hotforsecurity/fbi-issues-private-industry-notification-in-light-of-florida-water-plant-hack

[6] Pew Trusts: https://www.pewtrusts.org/en/research-and-analysis/blogs/stateline/2021/03/10/florida-hack-exposes-danger-to-water-systems
# CD Projekt RED Ransomware attack 08/02/21

## General

On the 8th of February 2021, CD Projekt RED suffered a ransomware attack [1].

CD Projekt RED posted about the incident on Twitter [1], stating: 

>“An unidentified actor gained unauthorized access to our internal network, collected certain data belonging to CD PROJEKT capital group, and left a ransom note the content of which we release to the public.” [1].

This article [3] by the BBC covers the attack and the context surrounding CD Projekt RED at that time. 

On the 10th of June 2021, CD Projekt RED posted a further post on Twitter regarding the incident [2]. This post [2] described their actions taken due to this attack and placed a large focus on the data of employees.

## Exploit

The ransomware used against CD Projekt RED was HelloKitty [4]. 

This article [4] describes concisely how the ransomware works, including that:

>“Specific encryption recipes and routines can vary across variants of HelloKitty. Generally speaking, they tend to use a combination of AES-256 and RSA-2048 or even NTRU+AES-128.” [4].

The severity of the ransomware was clarified by CD Projekt RED, who stated: “Although some devices in our network have been encrypted, our backups remain intact.” [1].

## STRIDE Model

Information Disclosure – Data was taken by the attackers [1, 2]. On the 10th of June 2021, CD Projekt RED stated: 

>“We are not able to confirm the exact contents of the data in question, though we believe it may include current/former employee and contractor details in addition to data related to our games.” [2].

Elevation of privilege – CD Projekt RED describes this in their Twitter post [1], stating: “An unidentified actor gained unauthorised access to our internal network” [1].

## References

[1] @CDPROJEKTRED, “Important Update.” Twitter. Feb. 9, 2021. https://twitter.com/CDPROJEKTRED/status/1359048125403590660 (accessed Sep. 28, 2023).

[2] @CDPROJEKTRED, “IMPORTANT UPDATE.” Twitter. Jun. 10, 2021. https://twitter.com/CDPROJEKTRED/status/1403059158065295361 (accessed Sep. 28, 2023).

[3] C. Criddle. “Cyberpunk 2077 makers CD Projekt hit by ransomware hack.” *BBC News*. Accessed: Sep. 28, 2023. [Online]. Available: https://www.bbc.co.uk/news/technology-55994787

[4] SentinelOne. “Hello Kitty.” *SentinelOne*. Accessed: Sep. 28, 2023. [Online]. Available: https://www.sentinelone.com/anthology/hello-kitty/# WannaCry Ransomware Attack (2017)

--------------------------------
# 2020 Twitter Accounts Compromised Attacks

## Overview
People were phished using "Phone Spear Phishing".

## Details of the Attack 
Compromised staff accounts.

## Reference to STRIDE
Spoofing (impersonating famous people).

Tampering (unauthorised posting of Tweets).

Repudiation (Denying posting from other people's accounts).



## Vulerability Identified
Social Engineering.

## Exploit Used
Spear phishing.
Account impersonation.
Fraudulent tweets.

## Type of Threat(s)
Threatening to leak private DM's of compromised accounts.


## Financial Implications
Lost of people lost money.

## CVE?
Could not find one as the main vulnerability was humans! (Social Engineering)

## References
(No date) The 2020 twitter hack – so many lessons to be learned. Available at: https://digitalcommons.kennesaw.edu/cgi/viewcontent.cgi?article=1089&amp;context=jcerp (Accessed: 02 October 2023). 

2020 twitter account hijacking (2023) Wikipedia. Available at: https://en.wikipedia.org/wiki/2020_Twitter_account_hijacking (Accessed: 28 September 2023). 

Thompson, N. (2020) How twitter survived its biggest hack-and plans to stop the next one, Wired. Available at: https://www.wired.com/story/inside-twitter-hack-election-plan/ (Accessed: 30 September 2023). 

Witman, P.D. and Mackelprang, S. (2022) The 2020 twitter hack -- so many lessons to be learned., Journal of Cybersecurity Education, Research and Practice. Available at: https://eric.ed.gov/?id=EJ1332789 (Accessed: 02 October 2023). 
# SolarWinds cyberattack (2020)

--------------------------------
### Who was attacked and when

SolarWinds is a software company who is most known for creating systems like 'Orion' which helps businesses to manage
their IT infrastructure.The SolarWinds attack was discovered in December of 2020. Unfortunately for SolarWinds, the 
attackers had gained access in September 2019, meaning their actions went undetected for 14 months. Nobelium, a 
potential nation-state hacker group, were identified by microsoft as being responsible.

### The negative impact of the attack

Due to the long duration of time that this hack was left undetected it allowed the previously mentioned attackers known
as Nobelium to steal data for 14 months from more than eighteen thousand organisations. This complete breach of GDPR 
then grew exponentially due to the fact that the attack allowed Nobelium to see the inner workings of Orion's users, 
thus compromising those who used the user's systems.

### Categorisation via the STRIDE model

**Spoofing**:
This was one of the main reasons this attack was able to go on for so long. Due to the hackers installing
malicious code into the orion system they were able to create a backdoor. Nobelium were then able to utilise this by
gaining access to the organisations' systems and managed to impersonate users on these systems.

**Tampering**:
Although the reason for this attack remains unkown it is EXTREMELY likely that database integrity was
compromised by this attack.

**Repudiation**:
It is evident that Nobelium was able to impersonate users within the orion's systems. This is evidence that they were
presumably able to mimic and/or deny transactions within systems, making them hidden.

**Information disclosure**:
Due to the large amount of time that the SolarWinds attack went uncovered, it is thought that thousands of customers
from thousands of organisations had there data stolen. The fact this went on for so long means that the attackers would
have been able to get far more data than if they were detected immediately and therefore for each customer there was
most likely months worth of data stolen.

**Denial of service**:
Due to the nature of this attack being to remain hidden and avoid suspicion, there is no evidence or reason to think
the attackers were planning to deny service as this would alert SolarWinds to the attack almost immediately.

**Elavation of privillege**:
Because of Nobelium's ability to impersonate users, they were able to act as admins to systems without ever revealling
themselves

### What measure have been put in place as a result of the attack. 
After the SolarWinds attack which was actually one of many in 2020, the U.S. CyberSecurity and Infrastructure Security
Agency saw first began by issuing guidance that helped defend against supply chain attacks like this one. It wasn't till
one year later that the biden administration put out an executive order mandating SBOMs which are used to quickly 
determine wherether they are at potential risk of a newly discovered vulnerability

### References
* https://www.techtarget.com/whatis/feature/SolarWinds-hack-explained-Everything-you-need-to-know#:~:text=On%20May%2027%2C%202021%2C%20Microsoft,more%20than%20150%20different%20organizations.
* https://www.businessinsider.com/solarwinds-hack-explained-government-agencies-cyber-security-2020-12?r=US&IR=T
* https://en.wikipedia.org/wiki/2020_United_States_federal_government_data_breach
* https://advisory.kpmg.us/blog/2020/what-are-sboms.html
# Introduction

Moonlight Maze was a cyber-espionage campaign that started in 1996, with the aim of stealing sensitive information from Government, Military, and Academic organisations in the United States and United Kingdom.
[[8](https://en.wikipedia.org/wiki/Moonlight_Maze)] Each attack would typically leverage publicly available exploits as well as heavily modified variants of publicly available malware.

Targets of Moonlight Maze include:
- NASA
- US Department of Energy
- US Navy
- US Army
- US Air Force
- US Department of Defense
- US National Oceanic and Atmospheric Administration
- UK Institute of Personnel and Development
- Various universities, research organisations, and academic organisations in the USA and UK
- Various public libraries in the USA
- Various organisations and businesses in Canada, Brazil, Germany, Norway, and Thailand [[10](https://medium.com/@chris_doman/the-first-sophistiated-cyber-attacks-how-operation-moonlight-maze-made-history-2adb12cc43f7)]

# Modus operandi of the attackers

The attackers would start by targeting specific web servers with an exploit against the vulnerable PHF binary, which was commonly located in the 'cgi-bin' directory of the web server. [[2](https://insecure.org/sploits/phf-cgi.html)]

This exploit allowed the attackers to exfiltrate the credentials of users [[2](https://insecure.org/sploits/phf-cgi.html)], usually via an FTP connection. [[3](https://securelist.com/penquins-moonlit-maze/77883/)]

Once the attackers gained access to the server using stolen credentials, they would attempt to gain root permissions by running various escalation of privilege exploits.

Once the attackers had root privileges, they would typically deploy network sniffers, exfiltration tools, and other malware onto the compromised server for several months. [[4](https://www.youtube.com/watch?v=jgTDvvl_j5Y)]

After exfiltrating as much data as possible for several weeks / months, the attackers would then survey the network of the compromised machine, in the hope of discovering more vulnerable servers on the network.

Typically, each server they compromised would be leveraged as a proxy / staging server for the attackers to tunnel their malicious activity through. [[3](https://securelist.com/penquins-moonlit-maze/77883/)] [[4](https://www.youtube.com/watch?v=jgTDvvl_j5Y)]

# Impact of the attack

Thousands of documents were stolen from dozens of organisations in the USA and UK.
These documents usually contained sensitive information relating to national security, military technologies, and encryption techniques. [[8](https://en.wikipedia.org/wiki/Moonlight_Maze)]

Not much is publicly known about exactly what information was stolen, due to the fact that the FBI has kept this information classified despite multiple Freedom of Information requests. [[1](https://www.youtube.com/watch?v=zSrjkWDXSS8)]

# Categorisation in the STRIDE Model
### Spoofing
The attackers used stolen credentials in order to imitate government personnel, military personnel, university students, and others. [[4](https://www.youtube.com/watch?v=jgTDvvl_j5Y)]

The attackers compromised servers in various countries (UK, Canada, Brazil, Germany, Norway, Thailand) and turned them into proxies for the purpose of spoofing malicious traffic to appear as if it was coming from a legitimate source.
### Tampering
The attackers used kernel patchers for SunOS systems in order to gain low level access to the system. (Probably for the purpose of hiding malicious activity) [[5](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07180254/Penquins_Moonlit_Maze_AppendixB.pdf)]
### Repudiation
The attackers used log cleaners in order to wipe traces of malicious activity [[5](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07180254/Penquins_Moonlit_Maze_AppendixB.pdf)]
### Information Disclosure
The attackers used network sniffers, keyloggers, and exfiltration scripts to steal thousands of documents over a period of 4 years [[1](https://www.youtube.com/watch?v=zSrjkWDXSS8)] [[4](https://www.youtube.com/watch?v=jgTDvvl_j5Y)]
### Denial of Service
N/A
### Escalation of Privilege
The attackers used various exploit bundles for SunOS and IRIX systems in order to gain root privileges [[5](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07180254/Penquins_Moonlit_Maze_AppendixB.pdf)]

# CVEs
### CVE-1999-0025 (Buffer overflow vulnerability against IRIX systems)
This is a buffer overflow vulnerability in the 'df' program, which allows an attacker to run arbitrary commands with root privileges [[6](https://www.exploit-db.com/exploits/19274)]

The 'df' program was designed for IRIX 5.x and IRIX 6.x systems.

The Moonlight Maze attackers used this exploit for escalation of privilege. [[5](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07180254/Penquins_Moonlit_Maze_AppendixB.pdf)]

### CVE-1999-0067 (RCE vulnerability in the PHF binary in httpd web servers)
This vulnerability allows an attacker to remotely execute commands by using shell metacharacters in a malicious url string [[2](https://insecure.org/sploits/phf-cgi.html)] [[7](https://www.cvedetails.com/cve/CVE-1999-0067/)]

The Moonlight Maze attackers used this exploit for compromising specific web servers. [[3](https://securelist.com/penquins-moonlit-maze/77883/)] [[4](https://www.youtube.com/watch?v=jgTDvvl_j5Y)]

# What measures have been put in place as a result of the attack
The FBI attempted to mitigate this attack by removing the vulnerable PHF binary from publicly available government and military web servers. [[4](https://www.youtube.com/watch?v=jgTDvvl_j5Y)]

The Metropolitan Police turned one of the infected servers in London against the attackers in an attempt to collect intelligence about the attack. [[3](https://securelist.com/penquins-moonlit-maze/77883/)] [[4](https://www.youtube.com/watch?v=jgTDvvl_j5Y)] [[9](https://www.zdnet.com/article/ancient-moonlight-maze-backdoor-remerges-as-modern-apt/)]

It was later revealed in the Snowden leaks that both Canadian and American authorities had conducted counter-intelligence operations against Moonlight Maze [[4](https://www.youtube.com/watch?v=jgTDvvl_j5Y)]
(Note: In the Snowden leaks, Moonlight Maze is referenced under the codenames "Storm Cloud" & "MAKERSMARK") [[9](https://www.zdnet.com/article/ancient-moonlight-maze-backdoor-remerges-as-modern-apt/)]

# Summary video
[Here is a video](https://www.youtube.com/watch?v=9RorL9y70GU) that summarises the Moonlight Maze attack.

# References

[1] Thomas Rid (2016) Back to the future - moonlight maze, YouTube. Available at: https://www.youtube.com/watch?v=zSrjkWDXSS8 (Accessed: October 24, 2022).

[2] /cgi-bin/phf vulnerability (no date). Available at: https://insecure.org/sploits/phf-cgi.html (Accessed: October 24, 2022).

[3] Costin Raiu, Thomas Rid, Juan Andres Guerrero-Saade, Daniel Moore (2017) Penquin's Moonlit Maze, Securelist. Kaspersky Lab. Available at: https://securelist.com/penquins-moonlit-maze/77883/ (Accessed: October 24, 2022).

[4] Costin Raiu, Thomas Rid, Juan Andres Guerrero-Saade, Daniel Moore (2017) A LINK TO THE PAST: CONNECTING THE BIRTH OF CYBERESPIONAGE Available at: https://www.youtube.com/watch?v=jgTDvvl_j5Y (Accessed: October 24, 2022).

[5] Costin Raiu, Thomas Rid, Juan Andres Guerrero-Saade, Daniel Moore (2017) Moonlight Maze Technical Report. Available at: https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2018/03/07180254/Penquins_Moonlit_Maze_AppendixB.pdf (Accessed: October 24, 2022).

[6] David Hedley (1997) SGI IRIX 6.3 - 'df' Local Privilege Escalation Available at: https://www.exploit-db.com/exploits/19274 (Accessed: October 24, 2022).

[7] (no author) (1996) Remote Command Execution Vulnerability in PHF CGI program Available at: https://www.cvedetails.com/cve/CVE-1999-0067/ (Accessed: October 24, 2022).

[8] Moonlight Maze - Wikipedia Available at: https://en.wikipedia.org/wiki/Moonlight_Maze (Accessed: October 24, 2022).

[9] Charlie Osborne (2017) Ancient Moonlight Maze backdoor remerges as modern APT Available at: https://www.zdnet.com/article/ancient-moonlight-maze-backdoor-remerges-as-modern-apt/ (Accessed: October 24, 2022).

[10] Chris Doman (2016) The First Cyber Espionage Attacks: How Operation Moonlight Maze made history Available at: https://medium.com/@chris_doman/the-first-sophistiated-cyber-attacks-how-operation-moonlight-maze-made-history-2adb12cc43f7 (Accessed: October 01, 2023)## Introduction
**Type:** Cyber Espionage \
**Description:** "The Shadow Net attack used internet services such as social networking and cloud computing platforms, including Twitter (now X), Google Groups, and Yahoo Mail, which were used to host and infect computers with malware." [1]

## Who Attacked and When
The Shadow Network cyber attack, also known as GhostNet, was attributed to hackers with ties to China. The attack was first uncovered in 2009, and there is no definitive proof of the exact individuals or entities responsible. However, "there was suspicion, but no confirmation, that one of the hackers had a connection to the University of Electronic Science and Technology in Chengdu. The account of another hacker was linked to a Chengdu resident who claimed to know little about the hacking." [1]

GhostNet was discovered in March 2009 and was followed by Shadow Net, which researchers "said was more sophisticated and difficult to detect." [1] Shadow Net was discovered on April 6, 2010, after eight months of research by individuals in Canada and the United States.

## Negative Impact of the Attack
The attack majorly impacted countries just beyond India. The data stolen from the compromised agencies "includes about 1,500 letters sent from the Dalai Lama's office between January and November 2009, reports on missile systems in India, and documents related to NATO force movements in Afghanistan." [2] This confidential information being potentially leaked to the Chinese government could have catastrophic impacts on any future conflict between the already hostile India and China. Access to important missile documentation could detrimentally effect the missiles effectiveness. Potentially compromising both the location and functionality of the missiles. The details regarding troop movement also could be catastrophic as the information could be passed on to opposing forces which could cost the lives of the NATO soldiers operating in Afghanistan.

## Vulnerability Exploited
The attackers behind the Shadow Network cyber attack leveraged several vulnerabilities and techniques, including:

1. Spear Phishing: The attack began with targeted spear-phishing emails sent to specific individuals within the Indian government and other organizations. These emails contained malicious attachments or links designed to compromise the recipient's system. [2]
2. Malware: Once a victim opened the malicious attachment or clicked on the link, malware was delivered and executed on their computer. This malware was responsible for establishing a backdoor connection to the attacker's command and control servers. They used exploits in Adobe PDFs and Microsoft Word 2003.
3. Command and Control (C&C) Servers: The attackers maintained C&C servers to communicate with compromised systems, allowing them to exfiltrate data, install additional malware, and maintain control over the infected systems.

## CVS Vulnerabilites
"The people behind the Shadow attacks used a variety of exploits and file types to compromise their victims. We observed the group using PDF, PPT, and DOC file formats to exploit Adobe Acrobat and Acrobat Reader, Microsoft Word 2003, and Microsoft PowerPoint 2003." [2]
1. [CVE-2009-0927](https://www.cvedetails.com/cve/CVE-2009-0927/) [3]
2. [CVE-2009-2990](https://www.cvedetails.com/cve/CVE-2009-2990/) [3]
3. [CVE-2009-4324](https://www.cvedetails.com/cve/CVE-2009-4324/) [3]

## STRIDE Model
**Spoofing**: Attackers used social engineering tactics in spear-phishing emails to impersonate trusted entities or individuals. \
**Tampering**: The attackers tampered with compromised systems by installing and executing malware, enabling unauthorised access and data exfiltration. \
**Repudiation**: This aspect is less relevant in cyber espionage cases like the Shadow Network attack. \
**Information Disclosure**: The primary goal of this attack was to steal sensitive information and documents from the Indian government and other organisations, constituting a significant information disclosure threat. \
**Denial of Service (DoS)**: The attack's primary focus was on data exfiltration rather than causing service disruptions. \
**Elevation of Privilege**: The attackers aimed to gain elevated privileges on compromised systems to access sensitive data.

## Measures in Place to Prevent Future Attacks
1. **Update Software:** The Indian government should update all their software to use the latest versions to ensure that all known vulnerabilities are patched. 
2. **Anti-Virus/Phishing Software:** Utilise software to determine if an attachment is potentially dangerous or contains malicious code. 
3. **Hosting Sensitive Information Securely:** All sensitive information should be kept in a safe location or server and should not be shared through email or other social networks.

## References
[1] Wikipedia. (2022). Shadow Network. [online] Available at: https://en.wikipedia.org/wiki/Shadow_Network [Accessed 25 Sep. 2023].
[2] CNET. (n.d.). Report: India targeted by spy network. [online] Available at: https://www.cnet.com/news/privacy/report-india-targeted-by-spy-network/ [Accessed 25 Sep. 2023].
[3] CVE Details (2009). CVE security vulnerability database. Security vulnerabilities, exploits, references and more. [online] Cvedetails.com. Available at: https://www.cvedetails.com/.
Reservation for 2015-16 Democratic National Committee cyber attacks
## 2009-2010 Stuxnet attacks on Iranian nuclear facilities
<br>

### What is Stuxnet 
Stuxnet is a powerful and malicious computer worm, which unlike any other virus or worm, escaped the digital realm to wreak physical destruction on the computers controlled, rather than simply hijacking the targets or stealing information from them. It is also reportedly the largest and costliest of this type of malware because not only it damaged Iran's uranium-enrichment centrifuges, but cyber attackers modified it over time and adapted it to target other establishments such as power plants and gas pipes.

### Who was attacked
The attacks which used the Stuxnet were aimed at Iran, in the hopes of delaying the country's nuclear proliferation without resorting to an airstrike or an attack by special operation forces and targeted Iran's uranium-enrichment programme, particularly IR-1 centrifuges at the Natanz plant [[1](https://www.researchgate.net/publication/323199431_Stuxnet)].

### When the attacks happened
The attacks happened in two waves. The first wave was less visible and more targeted than the second wave. It infected four Iranian organizations in June and July 2009. After that, and before Stuxnet’s public discovery, the malware’s operators tried to attack again in March. They targeted two organizations attacked earlier and a new one in April and May 2010, using updated code, which made the worm spread more aggressively [[2](https://isis-online.org/isis-reports/detail/stuxnet-malware-and-natanz-update-of-isis-december-22-2010-reportsupa-href1/#5)].

### The negative impact of the attacks
The Stuxnet worm reportedly infected more than 200,000 machines in 14 Iranian facilities and may have ruined up to 10% of the 9,000 centrifuges in Natanz, delaying the expected expansion of the plant, and further consuming a limited supply of centrifuges to replace those destroyed [[3](https://www.forenova.com/blog/deep-dive-into-advanced-persistent-threats)]. 

### The attack agent
While no country has officially admitted to creating Stuxnet, it is widely believed that the US and Israel jointly developed this cyberweapon and carried out the attacks [[4](https://en.wikipedia.org/wiki/Stuxnet)]. 

### CVEs
Stuxnet attacks against Iranian nuclear facilities targeted three systemic layers:
-	Windows OS
-	Siemens PCS 7, WinCC, and STEP7 industrial software applications
-	Siemens S7 programmable logic controller

And exploited the following vulnerabilities and exposures:
- ([**CVE-2010-2568**](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2010-2568)) served as the initial attack vector on a flash drive targeting Windows systems by allowing quick execution of a payload from a connected USB device.

- Windows Print spooler vulnerability ([**CVE-2010-2729**](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2010-2729)) which allowed remote attackers to create files in a system directory, and consequently execute arbitrary code, by sending a crafted print request over RPC.

- Siemens Simatic WinCC and PCS 7 SCADA system vulnerability ([**CVE-2010-2772**](https://cve.mitre.org/cgi-bin/cvename.cgi?name=cve-2010-2772)) which was the critical vulnerability that was exploited by the attackers to access the back-end database and gain privileges.

### Categorisation of threats in the STRIDE model
- **Denial of service** – the attackers rendered the centrifuges unusable  
- **Elevation of privilege** – the attackers performed privilege escalation to execute malicious code  
- **Tampering/Spoofing** – the attackers were able to send specially crafted print requests to the vulnerable systems that had print spooler interfaces exposed over RPC and then spoofed sensor signals so that the targeted systems won't shut down due to abnormal behaviour. 

### What measures have been put in place as a result of the attacks
There were several preventive and reactive countermeasures put in place as a result of the attacks.
Preventative countermeasures included effective security policies and procedures, security awareness programs within the organisations, disablement of all unnecessary services, software restriction policies, and the usage of comprehensive patch management programs.
The reactive countermeasures comprised of implementing a more secure IDS, adding passive vulnerability scanners, and implementing SCADA honeypots to help with the detection of potential malicious tampering.

### Other attacks that used the same vulnerabilities

 [CVE-2010-2568](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2010-2568) vulnerability remained extremely popular.
New families of malware such as Chmine, Vobfus, ZeuS and Sality quickly leveraged this flaw in order to propagate and infect systems which then were used to perform other attacks.

- The Vobfus worm was used to distribute a virus called 'Beebone'and in September 2014, attackers used this virus to take control of 100,000 computers in order to steal passwords and download other malicious programs [[5](https://www.bbc.co.uk/news/technology-32218381)]. 

- In February 2022, DDoS attacks were conducted against Ukrainian web forums using a botnet made of computers infected with Sality [[6](https://www.cisa.gov/uscert/ncas/alerts/aa22-110a)].


<br><br>
### References
[1] ResearchGate. (n.d.). (PDF) Stuxnet. [online] Available at: https://www.researchgate.net/publication/323199431_Stuxnet.

[2] David Albright, Paul Brannan, and Christina Walrond (2011). Stuxnet Malware and Natanz: Update of ISIS December 22, 2010 Report. [online] institute for science and international security. Available at: https://isis-online.org/isis-reports/detail/stuxnet-malware-and-natanz-update-of-isis-december-22-2010-reportsupa-href1/#5 [Accessed 4 Oct. 2022].

‌
[3] https://www.forenova.com/blog/deep-dive-into-advanced-persistent-threatswww.forenova.com. (n.d.). A Deep Dive into Advanced Persistent Threats (APT). [online] Available at: https://www.forenova.com/blog/deep-dive-into-advanced-persistent-threats [Accessed 4 Oct. 2022].

‌
[4] https://en.wikipedia.org/wiki/StuxnetWikipedia Contributors (2019). Stuxnet. [online] Wikipedia. Available at: https://en.wikipedia.org/wiki/Stuxnet.

‌
[5] Europol kills off shape-shifting ‘Mystique’ malware. (2015). BBC News. [online] 9 Apr. Available at: https://www.bbc.co.uk/news/technology-32218381 [Accessed 4 Oct. 2022].

‌
[6] www.cisa.gov. (2022). Russian State-Sponsored and Criminal Cyber Threats to Critical Infrastructure | CISA. [online] Available at: https://www.cisa.gov/uscert/ncas/alerts/aa22-110a.

‌

# 2022 LastPass Breach
---
## Overview
Lastpass suffered two breaches in 2022<sup>[1]</sup>, this page focuses on the one with greater impact, the second breach.

By targeting a DevOps engineer, unauthorized access was gained to cloud backups. These backups contained a wide range of data, including customer data (encyrpted and unencrypted)

## Details of attack
A DevOps engineers personal computer was targeted via their installation of Plex Media Server. The installation was many versions behind the most recent version at the time, and was missing security updates.<sup>[3]</sup>

The specific vulnerability targeted was [CVE-2020-5741](https://nvd.nist.gov/vuln/detail/CVE-2020-5741), which allowed arbitrary python code to be executed on the local system, from the current system user.<sup>[3]</sup>

To exploit this vulnerability, valid credentials for a Plex account are required by the attacker, these were leveraged in some way from the engineer<sup>[2][3]</sup>

The python code exeution was used to install keylogger software, which was used to capture the engineer's master password for their LastPass Vault. This vault contained secure notes describing how to access critical information for LastPass, and necessary decryption keys.<sup>[3]</sup>

LastPass became aware of the breach after warnings from Amazon of anomalous behaviour<sup>[4]</sup>

*Attacker Unknown*

## Categorisation

This is an attack with many steps, so can fall into multiple STRIDE catagories

- A **spoofing** attack was made on the DevOps engineer's Plex installation, and multiple other times, using the credentials extracted from the engineer's vault
- There was an attempt at **Tampering** when the attacker tried to access Cloud Identity and Access Management roles<sup>[4]</sup>
- There is a risk of **Information disclosure** as the information extracted during the breach could be made public, or sold. There's even a risk of some password being brute force decypted<sup>[6]</sup>


## Negative Impact
The data taken includes (but is not limited to):<sup>[5]</sup>

- **Encrypted** customer vaults
- Customer account secrets
	+ API keys
	+ One time pads
	+ Seeds for one time pads
- Customer information
	+ Including billing address, names and ip address
	+ Also including Information about Vault encryption configurations
	
LastPass operated with a breach-assumption policy<sup>[6]</sup>, meaning that all vault data is secure under the event of a breach. The vault is secured via 256 bit AES encryption using a hashed version of the master password as a key<sup>[7]</sup> and therefore the vault is only susceptible to a brute force attack. So long as a user uses a secure master password, and keeps their password secret, their vault data should be safe.

However, much of the other data extracted is still valuable for a malicious party

The key impact for LastPass will be the substantial reputation hit that they have taken, a password manager such as LastPass, that stores data on the cloud is dependent on customer trust, especially considering this is one of two data breaches that took place, so close together.
See [this](https://www.wired.com/story/lastpass-breach-vaults-password-managers/) article from WIRED, titled "It's Time to Ditch This Password Manager" (sorry about the paywall)


## Measures Implemented
Below is a list of measures that have been implemented as a result of this breach, from the LastPass incident report:<sup>[2]</sup>

<blockquote>

- With the assistance of Mandiant, we forensically imaged devices to investigate corporate and personal resources and gather evidence detailing potential threat actor activity.
- We assisted the DevOps Engineer with hardening the security of their home network and personal resources.
- We enabled Microsoft’s conditional access PIN-matching multifactor authentication using an upgrade to the Microsoft Authenticator application which became generally available during the incident.
- We rotated critical and high privilege credentials that were known to be available to the threat actor; we continue to rotate the remaining lower priority items that pose no risk to LastPass or our customers.
- We began revoking and re-issuing certificates obtained by the threat actor.
- We analyzed LastPass AWS S3 cloud-based storage resources and applied or started to apply additional S3 hardening measures:

	- We put in place additional logging and alerting across the Cloud Storage environment with tighter IAM policies enforced.
	- We deactivated prior development IAM users.
	- We enabled a policy that prevents the creation and use of long-lived development IAM users in the new development environment.
	- We rotated existing production service IAM user keys, applied tighter IP restrictions, and reconfigured policies to adhere to least privilege.
	- We deleted obsolete service IAM users from the development and production environments.
	- We are enabling IAM resource tagging enforcement on accounts for both users and roles with periodic reporting on non-compliant resources.

- We rotated critical SAML certificates used for internal and external services.
- We deleted obsolete/unused SAML certificates used for development, services, or third parties.
- We revised our 24x7 threat detection and response coverage, with additional managed and automated services enabled to facilitate appropriate escalation.
- We developed and enabled custom analytics that can detect ongoing abuse of AWS resources.
</blockquote>

## References

[1][Security incident update - LastPass](https://blog.lastpass.com/2023/03/security-incident-update-recommended-actions/)

[2][Incident additional details - LastPass](https://support.lastpass.com/s/document-item?bundleId=lastpass&topicId=LastPass/incident-2-details.html&_LANG=enus)

[3][LastPass Hack: Engineer's Failure to Update Plex Software Led to Massive Data Breach - Ravie Lakshmanan](https://support.lastpass.com/s/document-item?bundleId=lastpass&topicId=LastPass/incident-2-details.html&_LANG=enus)

[4][LastPass says employee’s home computer was hacked and corporate vault taken](https://arstechnica.com/information-technology/2023/02/lastpass-hackers-infected-employees-home-computer-and-stole-corporate-vault/)

[5][What data was accessed? - LastPass](https://support.lastpass.com/s/document-item?language=en_US&bundleId=lastpass&topicId=LastPass/incident-data.html&_LANG=enus)

[5][LastPass data breach - University of Washington](https://ciso.uw.edu/2022/12/23/lastpass-data-breach/)

[6][LastPass technical whitepaper - LastPass](https://support.lastpass.com/s/document-item?bundleId=lastpass&topicId=LastPass/lastpass_technical_whitepaper.html&_LANG=enus)


## Who was attacked and when

Adobe is a world-famous company producing several graphics, publishing, and printing applications. Even though Adobe is notorious for its products, it still had one of the most significant data breaches of the 21st century, impacting at least 38 million people worldwide. 
In January 2013, Adobe patched its web applications development tool ColdFusion, even though it took about two weeks to release a patch after informing users about the potential threat. (It is still unknown if it had to do anything with the breach from Adobe servers). The attack on the Adobe servers happened in mid-August, while the company noticed the breach only on 17th September and informed their customers about a potential threat more than two weeks later. As of October 2022, the exact vulnerability hackers used is still unknown. Before the official press release of Adobe's CSO Brad Arkin about the breach (Arkin, 2013), Brian Krebs, American journalist and investigator, informed Adobe of the fact that there was a dump with usernames, passwords, and credit card data of their services. This included source codes for their applications lying on a server where an unknown group of hackers stored their prey. KrebsOnSecurity also discovered that the hackers who earlier hacked into National White Collar Crime Centre, also known as NW3C, train state and local law enforcement to fight cybercrime. The same hackers' group used ColdFusion CVE-2013-0632 vulnerability, which Adobe patched, but NW3C did not patch their systems in time. We can assume that hackers somehow used ColdFusion vulnerability to breach Adobe servers. 

## The negative impact of the attack

Even though the breach was unnoticed for about a month, the attackers managed to download the source code for popular Adobe applications like Acrobat, Photoshop, BlueFusion and others. The potential outcome could be potential users installing in some way infected applications. Moreover, Adobe first stated that about 2.9 million encrypted passwords and credit card details were leaked. However, in a month, it turned out that these numbers could be up to twenty times higher. The tip of the iceberg in that leak is that all passwords were encrypted using the same key and used a semantically unsecure ECB cryptography method. To be precise, many passwords contained hints. While passwords were encrypted, they were not salted and did not have any nonce. This meant that the same passwords had the same encrypted forms. Even without deep analysis, by finding the identical hashes, it was possible to find users who used the same password and, if at least one of them left a "hint" for their password, allowed to guess passwords for all accounts. In this situation, it would be beneficial to use password salting to create unique passwords, as in this case, simple cryptanalysis could have been used to guess the most popular passwords. A solution making it harder to analyse passwords would be to atleast add salt to each password (Ducklin, 2013). 

## A categorisation in the STRIDE model

Even though the attackers' vulnerability is still unknown, the consequences have many threats.
For instance, leaked passwords could be bruteforced and decrypted, creating a spoofing issue. The data could be used for authorisation by third parties in Adobe services and other companies' accounts. Moreover, information was disclosed to third parties as a number of these passwords and encrypted credit card numbers appeared in the hands of third parties. 

## Academic Importance

Even though the Adobe data breach was one of the biggest in cyberattacks history, we can still learn many things from it. For instance, Business School student Sarah DesJardins mentioned Adobe 2013 breach in a different perspective in their dissertation (Desjardins, 2014). By analysing how this breach and other similar cyberattacks affected customers, employees, and the company's name, it is evident that cyberattacks can be very harmful from all viewpoints. Customers lose their information, and potential criminals could use their credit card details to buy expensive items. Their passwords could have been used on multiple websites. A breach like this allowed the author to show the importance of cybersecurity and how it can affect businesses, however, from cybersecurity point of view, Adobe has very much unsaid about how this breach could have happened.


## What measures have been put in place as a result of the attack.

After the incident, Adobe has taken a different approach to security. During one of the interviews, the CSO of Adobe, Brad Arkin, commented that the idea of keeping product engineering separate from IT security could not hold and improved security privately. The minimum they could have done is change the encryption method for passwords from ECB to a more secure one. 
As for communication with customers, Adobe has done pretty much to save their name. They reset all affected customers' passwords, so the leaked ones would not work on the Adobe products. This breach was so significant that other tech giants like Facebook had to inform their customers to change passwords if they used the same one for Adobe services. Moreover, Adobe privately informed customers whose credit card details could be leaked for them to take action.

## References

1. P. Ducklin, Anatomy of a password disaster – Adobe’s giant-sized cryptographic blunder. Naked Security (2013), (available at https://nakedsecurity.sophos.com/2013/11/04/anatomy-of-a-password-disaster-adobes-giant-sized-cryptographic-blunder/).
2. B. Krebs, Adobe To Announce Source Code, Customer Data Breach – Krebs on Security. Krebsonsecurity.com (2013), (available at https://krebsonsecurity.com/2013/10/adobe-to-announce-source-code-customer-data-breach/).
3. M. Mimoso, Adobe Breached, Acrobat and ColdFusion Code Stolen Along with 2.9M Customer Records. Threatpost.com (2013), (available at https://threatpost.com/adobe-breached-acrobat-and-coldfusion-code-stolen-along-with-2-9m-customer-records/102522/).
4. B. Arkin, Important Customer Security Announcement. Adobe Blog (2013), (available at https://blog.adobe.com/en/publish/2013/10/03/important-customer-security-announcement).
5. A. Hern, Did your Adobe password leak? Now you and 150m others can check. the Guardian (2013), (available at https://www.theguardian.com/technology/2013/nov/07/adobe-password-leak-can-check).
6. T. Bell, Adobe’s CSO talks security, the 2013 breach, and how he sets priorities. CSO Online (2018), (available at https://www.csoonline.com/article/3268035/adobe-s-cso-talks-security-the-2013-breach-and-how-he-sets-priorities.html#:~:text=It%20was%20one%20of%20the,from%2038%20million%20Adobe%20users).
7. S. Desjardins, thesis, University at Albany, State University of New York (2014).
8. B. Krebson, Data Broker Hackers Also Compromised NW3C – Krebs on Security. Krebsonsecurity.com (2013), (available at https://krebsonsecurity.com/2013/10/data-broker-hackers-also-compromised-nw3c/).

# **2017 NotPetya malware attack**

## The main attacked subject and time
Two years after the Ukraine power grid attack, Sandworm struck again, this time with a malware attack that, while almost certainly focused on Ukraine, inflicted enormous collateral damage across the globe. ESET estimated on **28 June 2017** that **80% of all infections were in Ukraine**, with **Germany** second hardest hit with about **9%**. 
Interestingly, although NotPetya gave the impression of being a ransomware attack, clues quickly began to suggest that the motives of its creators were more political than financial and that Ukraine was their main target. One such clue was the software used to initiate the infection was the **Ukrainian tax software, M.E.Doc,** which is used throughout the country. As a result, 80% of NotPetya infections were estimated to have occurred in Ukraine.

## Negative impact
NotPetya proved to be a more significant and virulent threat. Like the WannaCry ransomware that also caused global havoc in 2017, it utilised a Windows Server Message Block (SMB) exploit to spread more rapidly.It’s estimated that organisations collectively lost $1 billion as a result of the attack.

## STRIDE model
Tampering
Spoofing

## Vulnerability
MeDoc provides periodic updates to its program through an update server. On the day of the attack, 27 June 2017, an update for MeDoc was pushed out by the update server, following which the ransomware attack began to appear. British malware expert Marcus Hutchins claimed **"It looks like the software's automatic update system was compromised and used to download and run malware rather than updates for the software."** Following the initial 27 June attack, security experts found that the code that had infected the M.E.Doc update had a **backdoor** that could potentially be used to launch another cyberattack.

## The attack agent
Wired technology writer Andy Greenberg, in reviewing the history of the cyberattacks, said that the attacks came from a **Russian military hacker group** called "**Sandworm**".

## Exploit
The cyberattack was based on a modified version of the Petya ransomware. Like the WannaCry ransomware attack in May 2017, Petya uses the **EternalBlue exploit** previously discovered in older versions of the Microsoft Windows operating system. When Petya is executed, it encrypts the Master File Table of the hard drive and forces the computer to restart. 

## Measure as to against the attack
The campaign was followed by an extensive public attribution to Russia, which denied all allegations. No further publicly known measures were taken by the victims against Russia.

## CVE number
Petya exploits the vulnerability **CVE-2017-0144** in Microsoft’s implementation of the Server Message Block protocol. 

## Research [1]
How the ransomware attack the system and how to recover from it—decrypting the MFT, interrupting the encryption process, using some anti-spyware and formatting the infected hard drive and restoring the ﬁles from backups.


*Reference:
https://en.wikipedia.org/wiki/2017_Ukraine_ransomware_attacks <br />
https://www.historyhit.com/the-biggest-cyberattacks-in-history/ <br />
https://cyberlaw.ccdcoe.org/wiki/NotPetya_(2017) <br />
[1]https://www.researchgate.net/publication/324489505_What_PetyaNotPetya_Ransomware_Is_and_What_Its_Remidiations_Are
*
