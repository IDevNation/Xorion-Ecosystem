// Alchemy API Configuration
const ALCHEMY_API_KEY = "khSccO9SmGDg3HOTyWK82";
const ETH_RPC_URL = `https://eth-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}`;

var wallets = [];
var currentWalletId = null;
var currentNetwork = 'xos-mainnet';

function getEl(id) { return document.getElementById(id); }

function saveData() {
  var theme = document.documentElement.getAttribute('data-theme');
  chrome.storage.local.set({ 
    wallets: wallets, 
    currentWalletId: currentWalletId, 
    theme: theme,
    currentNetwork: currentNetwork
  });
}

function loadData() {
  chrome.storage.local.get(['wallets', 'currentWalletId', 'theme', 'currentNetwork'], function(res) {
    if (res.theme === 'dark') toggleTheme(true);
    if (res.currentNetwork) currentNetwork = res.currentNetwork;
    
    if (res.wallets && res.wallets.length > 0) {
      wallets = res.wallets;
      if (res.currentWalletId) {
        currentWalletId = res.currentWalletId;
        showScreen('dashboard-screen');
        updateDashboard();
      } else {
        showScreen('lock-screen');
      }
    } else {
      showScreen('welcome-screen');
    }
  });
}

async function fetchRealBalance(address) {
    if (!address || !address.startsWith('0x')) {
        return "0 ETH";
    }
    try {
        const response = await fetch(ETH_RPC_URL, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                jsonrpc: '2.0',
                method: 'eth_getBalance',
                params: [address, 'latest'],
                id: 1
            })
        });
        const data = await response.json();
        if (data.result) {
            const balanceWei = parseInt(data.result, 16);
            const balanceEth = (balanceWei / 1e18).toFixed(6);
            return balanceEth + " ETH";
        }
        return "0 ETH";
    } catch(e) {
        console.error("Balance error:", e);
        return "Error";
    }
}

document.addEventListener('DOMContentLoaded', function() {
  loadData();

  if(getEl('theme-toggle')) getEl('theme-toggle').onclick = function() { toggleTheme(); };
  if(getEl('btn-create')) getEl('btn-create').onclick = function() { generateSeed(); showScreen('create-screen'); };
  if(getEl('btn-import')) getEl('btn-import').onclick = function() { showScreen('import-screen'); };
  if(getEl('btn-back-create')) getEl('btn-back-create').onclick = function() { showScreen('welcome-screen'); };
  if(getEl('btn-back-import')) getEl('btn-back-import').onclick = function() { showScreen('welcome-screen'); };
  if(getEl('btn-save-new')) getEl('btn-save-new').onclick = createWallet;
  if(getEl('btn-save-import')) getEl('btn-save-import').onclick = importWallet;
  if(getEl('btn-lock')) getEl('btn-lock').onclick = lockWallet;
  if(getEl('btn-unlock')) getEl('btn-unlock').onclick = unlockWallet;
  if(getEl('btn-copy')) getEl('btn-copy').onclick = copyAddress;
  if(getEl('btn-send')) getEl('btn-send').onclick = sendTransaction;
  if(getEl('btn-receive')) getEl('btn-receive').onclick = function() { showScreen('receive-screen'); generateQR(); };
  if(getEl('btn-back-receive')) getEl('btn-back-receive').onclick = function() { showScreen('dashboard-screen'); };
  if(getEl('btn-copy-recv')) getEl('btn-copy-recv').onclick = copyAddress;
  if(getEl('btn-new-wallet')) getEl('btn-new-wallet').onclick = function() { generateSeed(); showScreen('create-screen'); };
  if(getEl('btn-delete-wallet')) getEl('btn-delete-wallet').onclick = deleteWallet;
  if(getEl('wallet-selector')) getEl('wallet-selector').onchange = function(e) { currentWalletId = e.target.value; saveData(); updateDashboard(); };
  if(getEl('network-selector')) getEl('network-selector').onchange = function(e) { 
    currentNetwork = e.target.value; 
    saveData(); 
    updateDashboard(); 
  };
});

function toggleTheme(forceDark) {
  var html = document.documentElement;
  var isDark = (forceDark !== undefined) ? forceDark : (html.getAttribute('data-theme') !== 'dark');
  html.setAttribute('data-theme', isDark ? 'dark' : 'light');
  var btn = getEl('theme-toggle');
  if(btn) btn.innerText = isDark ? '☀️' : '🌙';
  if (forceDark === undefined) saveData();
}

function showScreen(id) {
  var screens = document.querySelectorAll('.screen');
  for (var i = 0; i < screens.length; i++) {
    screens[i].classList.remove('active');
  }
  var target = getEl(id);
  if(target) target.classList.add('active');
}

function generateSeed() {
  var words = ['abandon','ability','able','about','above','absent','absorb','abstract','absurd','abuse','access','accident','account','achieve','acid','acoustic','acquire','across','act','action','actor','actress','actual','adapt','add','addict','address','adjust','admit','adult','advance','advice','aerobic','affair','afford','afraid','again','age','agent','agree','ahead','aim','air','airport','aisle','alarm','album','alcohol','alert','alien','all','alley','allow','almost','alone','alpha','already','also','alter','always','amateur','amazing','among','amount','amused','analyst','anchor','ancient','anger','angle','angry','animal','ankle','announce','annual','another','answer','antenna','antique','anxiety','any','apart','apology','appear','apple','approve','april','arch','arctic','area','arena','argue','arm','armed','armor','army','around','arrange','arrest','arrive','arrow','art','artefact','artist','artwork','ask','aspect','assault','asset','assist','assume','asthma','athlete','atom','attack','attend','attitude','attract','auction','audit','august','aunt','author','auto','autumn','average','avocado','avoid','awake','aware','away','awesome','awful','awkward','axis'];
  var arr = [];
  for (var i = 0; i < 12; i++) {
    arr.push(words[Math.floor(Math.random() * words.length)]);
  }
  var seedStr = arr.join(' ');
  var disp = getEl('seed-display');
  if(disp) disp.innerText = seedStr;
  return seedStr;
}

function createWallet() {
  var p1 = getEl('new-pass').value;
  var p2 = getEl('confirm-pass').value;
  if (p1 !== p2 || p1.length < 4) { alert('Password mismatch or too short!'); return; }
  
  var seed = getEl('seed-display').innerText;
  var newWallet = {
    id: Date.now().toString(),
    name: 'Wallet ' + (wallets.length + 1),
    password: p1,
    networks: {
      'eth': { address: '0x' + Math.random().toString(16).substring(2,10), balance: 0.00, txs: [], symbol: 'ETH' },
      'bsc': { address: '0x' + Math.random().toString(16).substring(2,10), balance: 0.00, txs: [], symbol: 'BNB' },
      'polygon': { address: '0x' + Math.random().toString(16).substring(2,10), balance: 0.00, txs: [], symbol: 'MATIC' },
      'arbitrum': { address: '0x' + Math.random().toString(16).substring(2,10), balance: 0.00, txs: [], symbol: 'ARB' },
      'optimism': { address: '0x' + Math.random().toString(16).substring(2,10), balance: 0.00, txs: [], symbol: 'OP' },
      'xos-mainnet': { address: 'XOS' + Math.random().toString(36).substring(2,10).toUpperCase(), balance: 1000.00, txs: [], symbol: 'XOR' }
    },
    seed: seed
  };
  
  wallets.push(newWallet);
  currentWalletId = newWallet.id;
  saveData();
  updateDashboard();
  showScreen('dashboard-screen');
  getEl('new-pass').value = '';
  getEl('confirm-pass').value = '';
}

function importWallet() {
  var seed = getEl('import-seed-input').value.trim();
  var pass = getEl('import-pass').value;
  if (seed.split(' ').length !== 12) { alert('Invalid seed (must be 12 words)'); return; }
  if (pass.length < 4) { alert('Password too short'); return; }
  
  var newWallet = {
    id: Date.now().toString(),
    name: 'Imported ' + (wallets.length + 1),
    password: pass,
    networks: {
      'eth': { address: '0x' + Math.random().toString(16).substring(2,10), balance: 0.00, txs: [], symbol: 'ETH' },
      'bsc': { address: '0x' + Math.random().toString(16).substring(2,10), balance: 0.00, txs: [], symbol: 'BNB' },
      'polygon': { address: '0x' + Math.random().toString(16).substring(2,10), balance: 0.00, txs: [], symbol: 'MATIC' },
      'arbitrum': { address: '0x' + Math.random().toString(16).substring(2,10), balance: 0.00, txs: [], symbol: 'ARB' },
      'optimism': { address: '0x' + Math.random().toString(16).substring(2,10), balance: 0.00, txs: [], symbol: 'OP' },
      'xos-mainnet': { address: 'XOS' + Math.random().toString(36).substring(2,10).toUpperCase(), balance: 0.00, txs: [], symbol: 'XOR' }
    },
    seed: seed
  };
  
  wallets.push(newWallet);
  currentWalletId = newWallet.id;
  saveData();
  updateDashboard();
  showScreen('dashboard-screen');
  getEl('import-seed-input').value = '';
  getEl('import-pass').value = '';
}

function deleteWallet() {
  if (wallets.length <= 1) { alert('Cannot delete the only wallet!'); return; }
  if (!confirm('Are you sure? This cannot be undone.')) return;
  
  var newWallets = [];
  for (var i = 0; i < wallets.length; i++) {
    if (wallets[i].id !== currentWalletId) newWallets.push(wallets[i]);
  }
  wallets = newWallets;
  currentWalletId = wallets[0].id;
  saveData();
  updateDashboard();
}

function lockWallet() {
  currentWalletId = null;
  var passIn = getEl('unlock-pass');
  var errEl = getEl('unlock-error');
  if(passIn) passIn.value = '';
  if(errEl) errEl.innerText = '';
  showScreen('lock-screen');
  saveData();
}

function unlockWallet() {
  var passInput = getEl('unlock-pass').value;
  var errorEl = getEl('unlock-error');
  
  if (!passInput) {
    if(errorEl) errorEl.innerText = 'Enter password';
    return;
  }
  
  var foundWallet = null;
  for(var i=0; i<wallets.length; i++) {
    if(wallets[i].password === passInput) {
      foundWallet = wallets[i];
      break;
    }
  }
  
  if (foundWallet) {
    currentWalletId = foundWallet.id;
    saveData();
    updateDashboard();
    showScreen('dashboard-screen');
  } else {
    if(errorEl) errorEl.innerText = 'Incorrect password!';
  }
}

async function updateDashboard() {
  var wallet = null;
  for (var i = 0; i < wallets.length; i++) {
    if (wallets[i].id === currentWalletId) { wallet = wallets[i]; break; }
  }
  if (!wallet) return;
  
  var netData = wallet.networks[currentNetwork];
  if(!netData) {
    netData = { balance: 0, address: '...', txs: [], symbol: 'UNK' };
  }

  var netSelector = getEl('network-selector');
  if(netSelector) netSelector.value = currentNetwork;

  var walSelector = getEl('wallet-selector');
  if(walSelector) {
    walSelector.innerHTML = '';
    for (var j = 0; j < wallets.length; j++) {
      var w = wallets[j];
      var opt = document.createElement('option');
      opt.value = w.id;
      opt.innerText = w.name;
      if (w.id === currentWalletId) opt.selected = true;
      walSelector.appendChild(opt);
    }
  }

  // Real balance fetch for ETH
  if (currentNetwork === 'eth' && netData.address && netData.address.startsWith('0x')) {
    var realBalance = await fetchRealBalance(netData.address);
    var balEl = getEl('balance-display');
    if(balEl) {
      var match = realBalance.match(/^([\d.]+)/);
      if(match) balEl.innerText = match[1];
      var symSpan = balEl.parentElement?.querySelector('.currency-symbol');
      if(symSpan) symSpan.innerText = 'ETH';
    }
  } else {
    var balEl2 = getEl('balance-display');
    if(balEl2) balEl2.innerText = netData.balance.toFixed(4);
    var symSpan2 = balEl2?.parentElement?.querySelector('.currency-symbol');
    if(symSpan2) symSpan2.innerText = netData.symbol;
  }

  var addrEl = getEl('address-display');
  var recvAddrEl = getEl('receive-addr');
  if(addrEl) addrEl.innerText = netData.address;
  if(recvAddrEl) recvAddrEl.innerText = netData.address;

  var netNameEl = getEl('network-name');
  if(netNameEl) netNameEl.innerText = currentNetwork.toUpperCase();

  var list = getEl('tx-list');
  if(list) {
    list.innerHTML = '';
    if (!netData.txs || netData.txs.length === 0) {
      list.innerHTML = '<li style="text-align:center;padding:10px;">No transactions</li>';
    } else {
      for (var k = netData.txs.length - 1; k >= 0; k--) {
        var tx = netData.txs[k];
        var li = document.createElement('li');
        var sign = (tx.type === 'Sent') ? '-' : '+';
        li.innerHTML = '<div><span>' + tx.type + '</span><span>' + sign + tx.amount + ' ' + netData.symbol + '</span></div><div>' + (tx.to || 'Received') + '</div>';
        list.appendChild(li);
      }
    }
  }
  
  var qrContainer = getEl('qrcode');
  if(qrContainer && qrContainer.innerHTML !== '') generateQR();
}

function copyAddress() {
  var wallet = null;
  for (var i = 0; i < wallets.length; i++) {
    if (wallets[i].id === currentWalletId) { wallet = wallets[i]; break; }
  }
  if(!wallet) return;
  var addr = wallet.networks[currentNetwork].address;
  
  var tempInput = document.createElement("input");
  tempInput.value = addr;
  document.body.appendChild(tempInput);
  tempInput.select();
  document.execCommand("copy");
  document.body.removeChild(tempInput);
  
  alert('Address Copied!');
}

function generateQR() {
  var container = getEl('qrcode');
  if(!container) return;
  container.innerHTML = '';
  
  var wallet = null;
  for (var i = 0; i < wallets.length; i++) {
    if (wallets[i].id === currentWalletId) { wallet = wallets[i]; break; }
  }
  if(!wallet) return;
  
  var addr = wallet.networks[currentNetwork].address;
  
  if (typeof QRCode !== 'undefined') {
    new QRCode(container, {
      text: addr,
      width: 180,
      height: 180,
      colorDark : "#000000",
      colorLight : "#ffffff",
      correctLevel : QRCode.CorrectLevel.H
    });
  } else {
    container.innerText = "QR Library not loaded";
  }
}

function sendTransaction() {
  var to = getEl('send-to').value;
  var amtVal = getEl('send-amt').value;
  var amt = parseFloat(amtVal);
  
  var wallet = null;
  for (var i = 0; i < wallets.length; i++) {
    if (wallets[i].id === currentWalletId) { wallet = wallets[i]; break; }
  }
  if(!wallet) return;
  
  var netData = wallet.networks[currentNetwork];

  if (!to || !amt || amt <= 0) { alert('Invalid details'); return; }
  
  // Balance check removed for testing - will be re-enabled later
  // if (amt > netData.balance) { alert('Insufficient balance'); return; }

  netData.balance -= amt;
  netData.txs.push({ type: 'Sent', amount: amt, to: to, date: Date.now() });

  saveData();
  updateDashboard();
  alert('Transaction Successful!');
  
  var sTo = getEl('send-to');
  var sAmt = getEl('send-amt');
  if(sTo) sTo.value = '';
  if(sAmt) sAmt.value = '';
}