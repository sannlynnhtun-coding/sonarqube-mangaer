pub fn text(mm: bool, english: &str) -> &str {
    if !mm {
        return english;
    }
    match english {
        "Light mode" => "အလင်းစနစ်",
        "Dark mode" => "အမှောင်စနစ်",
        "Windows service:" => "Windows service:",
        "Web: Checking" => "Web: စစ်ဆေးနေသည်",
        "Web: Ready" => "Web: အသင့်ဖြစ်ပါပြီ",
        "Web: Starting" => "Web: စတင်နေသည်",
        "Web: Unavailable" => "Web: မရရှိသေးပါ — မှတ်တမ်း စစ်ဆေးပါ",
        "Web: Not ready" => "Web: အသင့်မဖြစ်သေးပါ",
        "Web: Not running" => "Web: အလုပ်မလုပ်သေးပါ",
        "Web: Database migration required" => "Web: Database migration လိုအပ်သည်",
        "Service is already stopped." => "Service ရပ်ထားပြီး ဖြစ်ပါသည်။",
        "Service start requested. SonarQube web server may still be initializing." => {
            "Service စတင်ရန် တောင်းဆိုပြီးပါပြီ။ SonarQube web server ပြင်ဆင်နေဆဲ ဖြစ်နိုင်ပါသည်။"
        }
        "Service stop requested." => "Service ရပ်ရန် တောင်းဆိုပြီးပါပြီ။",
        "Service removed." => "Service ဖယ်ရှားပြီးပါပြီ။",
        "Use existing service" => "ရှိပြီးသား Service ချိတ်ဆက်ရန်",
        "Existing service connected. Use Start / Stop / Restart. Configuration was not changed." => {
            "ရှိပြီးသား Service ချိတ်ဆက်ပြီးပါပြီ။ စတင် / ရပ် / ပြန်စတင် ကို အသုံးပြုနိုင်ပါပြီ။ ဆက်တင်များကို မပြောင်းထားပါ။"
        }
        "Show manager" => "Manager ပြန်ဖွင့်ရန်",
        "Exit" => "App မှ ထွက်ရန်",
        "Stop before editing settings. Close (×) hides the manager in the system tray. Right-click the tray icon for controls or Exit." => {
            "ဆက်တင်မပြောင်းမီ Service ကို ရပ်ပါ။ × နှိပ်လျှင် Manager သည် System tray ထဲတွင် ရှိနေမည်။ Tray icon ကို ညာကလစ်နှိပ်၍ Service ထိန်းချုပ်ရန် သို့မဟုတ် App မှ ထွက်ရန် ရွေးနိုင်ပါသည်။"
        }
        "5. Close this window to keep the manager in the system tray. Double-click its tray icon to reopen it. Right-click for Start, Stop, Restart and Exit. Exit closes only the manager; the service keeps running." => {
            "၅။ ဝင်းဒိုးပိတ်လျှင် Manager သည် System tray ထဲတွင် ရှိနေမည်။ Tray icon ကို နှစ်ချက်နှိပ်၍ ပြန်ဖွင့်ပါ။ ညာကလစ်နှိပ်၍ စတင်၊ ရပ်၊ ပြန်စတင် သို့မဟုတ် App မှ ထွက်နိုင်ပါသည်။ App မှ ထွက်သော်လည်း SonarQube service ဆက်အလုပ်လုပ်နေပါမည်။"
        }
        "Setup instructions" => "အသုံးပြုနည်း အဆင့်ဆင့်",
        "WINDOWS SERVICE · Start automatically when Windows starts" => {
            "WINDOWS SERVICE · Windows တက်လာလျှင် အလိုအလျောက် စတင်မည်"
        }
        "Refresh status" => "အခြေအနေ ပြန်စစ်ရန်",
        "SonarQube installation folder" => "SonarQube တည်ရှိရာ ဖိုလ်ဒါ",
        "Browse…" => "ဖိုလ်ဒါ ရွေးရန်…",
        "Host IP" => "Host IP လိပ်စာ",
        "Port" => "Port နံပါတ်",
        "Save settings" => "ဆက်တင် သိမ်းရန်",
        "Install service" => "Service တပ်ဆင်ရန်",
        "Start" => "စတင်ရန်",
        "Stop" => "ရပ်ရန်",
        "Restart" => "ပြန်စတင်ရန်",
        "Remove service" => "Service ဖယ်ရှားရန်",
        "Open SonarQube" => "SonarQube ဖွင့်ရန်",
        "View log" => "မှတ်တမ်း ကြည့်ရန်",
        "Running" => "အလုပ်လုပ်နေသည်",
        "Stopped" => "ရပ်ထားသည်",
        "Starting" => "စတင်နေသည်",
        "Stopping" => "ရပ်တန့်နေသည်",
        "Not installed" => "မတပ်ဆင်ရသေးပါ",
        "Press Refresh to check service" => "Service အခြေအနေ စစ်ရန် ပြန်စစ်ရန် ခလုတ်ကို နှိပ်ပါ",
        "Choose your SonarQube folder, then save and install the service." => {
            "SonarQube ဖိုလ်ဒါ ရွေးပြီး ဆက်တင်သိမ်းကာ Service တပ်ဆင်ပါ။"
        }
        "Settings saved. Start the service to apply them." => {
            "ဆက်တင် သိမ်းပြီးပါပြီ။ အသုံးပြုရန် Service ကို စတင်ပါ။"
        }
        "Opened SonarQube in your browser." => "Browser တွင် SonarQube ဖွင့်ပြီးပါပြီ။",
        "Background operation ended unexpectedly" => "နောက်ခံလုပ်ဆောင်မှု မမျှော်လင့်ဘဲ ရပ်သွားပါသည်",
        "127.0.0.1 = this computer only · 0.0.0.0 = all IPv4 interfaces. LAN access needs your firewall configuration." => {
            "127.0.0.1 = ဤစက်တွင်သာ သုံးရန် · 0.0.0.0 = IPv4 ကွန်ရက်အားလုံး။ LAN မှ ဝင်သုံးရန် Firewall ကို သတ်မှတ်ပေးပါ။"
        }
        "Stop before editing settings. Install service also starts SonarQube. Closing this window leaves the service running." => {
            "ဆက်တင်မပြောင်းမီ Service ကို ရပ်ပါ။ Service တပ်ဆင်လျှင် SonarQube ကိုပါ စတင်ပေးပါမည်။ ဤဝင်းဒိုးပိတ်သော်လည်း Service ဆက်အလုပ်လုပ်နေပါမည်။"
        }
        "1. Install Java, then extract SonarQube into C:\\sonarqube. This folder must directly contain conf, bin and lib. If the ZIP creates a versioned subfolder, select that subfolder with Browse." => {
            "၁။ Java တပ်ဆင်ပြီး SonarQube ZIP ကို C:\\sonarqube တွင် ဖြည်ပါ။ ရွေးထားသော ဖိုလ်ဒါအောက်၌ conf၊ bin နှင့် lib တိုက်ရိုက် ရှိရပါမည်။ ဗားရှင်းအမည်ပါ ဖိုလ်ဒါခွဲထဲ၌ ရှိလျှင် Browse ဖြင့် ထိုဖိုလ်ဒါခွဲကို ရွေးပါ။"
        }
        "2. Choose the folder in the manager. Keep host 127.0.0.1 for this computer and port 9000, or enter your own values. Use 0.0.0.0 for LAN access and configure your firewall separately." => {
            "၂။ App တွင် SonarQube ဖိုလ်ဒါကို ရွေးပါ။ ဤစက်တွင်သာ သုံးရန် Host ကို 127.0.0.1၊ Port ကို 9000 ထားနိုင်ပါသည်။ LAN မှ သုံးရန် Host ကို 0.0.0.0 ထားပြီး Firewall ကို သတ်မှတ်ပေးပါ။"
        }
        "3. Click Save settings, then Install service. Installation starts SonarQube and enables automatic startup with Windows." => {
            "၃။ ဆက်တင် သိမ်းရန် ကိုနှိပ်ပြီး Service တပ်ဆင်ရန် ကိုနှိပ်ပါ။ SonarQube စတင်လာပြီး Windows တက်လာတိုင်း အလိုအလျောက် စတင်ပါမည်။"
        }
        "4. Wait for Running, then click Open SonarQube. The server may still be initializing; if the page is unavailable, check View log and wait." => {
            "၄။ အလုပ်လုပ်နေသည် ဟုပြသည်အထိ စောင့်ပြီး SonarQube ဖွင့်ရန် ကိုနှိပ်ပါ။ Server ပြင်ဆင်နေဆဲ ဖြစ်နိုင်သောကြောင့် စာမျက်နှာမပေါ်လျှင် မှတ်တမ်း ကြည့်ရန် ကိုနှိပ်ပြီး ခဏစောင့်ပါ။"
        }
        "5. You can close the manager. The service keeps running and starts automatically next time Windows starts." => {
            "၅။ App ကို ပိတ်နိုင်ပါပြီ။ Service ဆက်အလုပ်လုပ်နေမည်ဖြစ်ပြီး Windows ပြန်တက်လာလျှင်လည်း အလိုအလျောက် စတင်ပါမည်။"
        }
        "To change settings later: Stop the service, save your changes, then Start. To change installation folders, remove the stopped service first." => {
            "နောက်ပိုင်း ဆက်တင်ပြောင်းရန် Service ကို ရပ်၊ ဆက်တင်သိမ်းပြီး ပြန်စတင်ပါ။ ဖိုလ်ဒါပြောင်းရန် ရပ်ထားသော Service ကို အရင်ဖယ်ရှားပါ။"
        }
        _ => english,
    }
}
