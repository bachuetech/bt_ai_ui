const eAppName = document.getElementById('app_name');
eAppName.textContent = application_title_name;

var isMicMuted = true;
const clipboardIcon = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-clipboard" viewBox="0 0 16 16">
<path d="M4 1.5H3a2 2 0 0 0-2 2V14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V3.5a2 2 0 0 0-2-2h-1v1h1a1 1 0 0 1 1 1V14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3.5a1 1 0 0 1 1-1h1v-1z"/>
<path d="M9.5 1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-1a.5.5 0 0 1 .5-.5h3zm-3-1A1.5 1.5 0 0 0 5 1.5v1A1.5 1.5 0 0 0 6.5 4h3A1.5 1.5 0 0 0 11 2.5v-1A1.5 1.5 0 0 0 9.5 0h-3z"/>
</svg>`

const megaphoneIcon = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-megaphone" viewBox="0 0 16 16">
  <path d="M13 2.5a1.5 1.5 0 0 1 3 0v11a1.5 1.5 0 0 1-3 0v-.214c-2.162-1.241-4.49-1.843-6.912-2.083l.405 2.712A1 1 0 0 1 5.51 15.1h-.548a1 1 0 0 1-.916-.599l-1.85-3.49-.202-.003A2.014 2.014 0 0 1 0 9V7a2.02 2.02 0 0 1 1.992-2.013 75 75 0 0 0 2.483-.075c3.043-.154 6.148-.849 8.525-2.199zm1 0v11a.5.5 0 0 0 1 0v-11a.5.5 0 0 0-1 0m-1 1.35c-2.344 1.205-5.209 1.842-8 2.033v4.233q.27.015.537.036c2.568.189 5.093.744 7.463 1.993zm-9 6.215v-4.13a95 95 0 0 1-1.992.052A1.02 1.02 0 0 0 1 7v2c0 .55.448 1.002 1.006 1.009A61 61 0 0 1 4 10.065m-.657.975 1.609 3.037.01.024h.548l-.002-.014-.443-2.966a68 68 0 0 0-1.722-.082z"/>
</svg>`

const micMuted = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-mic-mute" viewBox="0 0 16 16">
  <path d="M13 8c0 .564-.094 1.107-.266 1.613l-.814-.814A4 4 0 0 0 12 8V7a.5.5 0 0 1 1 0zm-5 4c.818 0 1.578-.245 2.212-.667l.718.719a5 5 0 0 1-2.43.923V15h3a.5.5 0 0 1 0 1h-7a.5.5 0 0 1 0-1h3v-2.025A5 5 0 0 1 3 8V7a.5.5 0 0 1 1 0v1a4 4 0 0 0 4 4m3-9v4.879l-1-1V3a2 2 0 0 0-3.997-.118l-.845-.845A3.001 3.001 0 0 1 11 3"/>
  <path d="m9.486 10.607-.748-.748A2 2 0 0 1 6 8v-.878l-1-1V8a3 3 0 0 0 4.486 2.607m-7.84-9.253 12 12 .708-.708-12-12z"/>
</svg>`

const microphone = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-mic" viewBox="0 0 16 16">
  <path d="M3.5 6.5A.5.5 0 0 1 4 7v1a4 4 0 0 0 8 0V7a.5.5 0 0 1 1 0v1a5 5 0 0 1-4.5 4.975V15h3a.5.5 0 0 1 0 1h-7a.5.5 0 0 1 0-1h3v-2.025A5 5 0 0 1 3 8V7a.5.5 0 0 1 .5-.5"/>
  <path d="M10 8a2 2 0 1 1-4 0V3a2 2 0 1 1 4 0zM8 0a3 3 0 0 0-3 3v5a3 3 0 0 0 6 0V3a3 3 0 0 0-3-3"/>
</svg>`

const textBoxBaseHeight = 40;  // This should match the default height set in CSS

var t2s = new txt2Speech();

//Function to sent the focus in a particular element. 
function autoFocusInput() {
  const userInput = document.getElementById('user-input');
  userInput.focus();
}

function speechDisable() {
  let micButton = document.getElementById('mic-button');
  isMicMuted = true;
  micButton.innerHTML = micMuted;
  micButton.classList.remove('btn-outline-danger');
  micButton.classList.remove('btn-primary');
  micButton.classList.add('btn-dark');
  micButton.disabled = true;
  micButton.setAttribute('aria-pressed',false);
}

function speechHandler() {
if (!isMicMuted) {
  let micButton = document.getElementById('mic-button');
  isMicMuted = true;
  micButton.innerHTML = micMuted;
  micButton.classList.remove('btn-outline-danger');
  micButton.classList.add('btn-primary');
}

}

var s2t = new SpeechRecognizer("user-input", speechHandler);
function micRequest(){
let micButton = document.getElementById('mic-button');
if (isMicMuted){
  isMicMuted = false;
  document.getElementById('mic-button').innerHTML = microphone;
  micButton.classList.remove('btn-primary');
  micButton.classList.add('btn-outline-danger');
  micButton.setAttribute('aria-pressed',true);
  //startSpeechRecognition("user-input");
  s2t.startSpeechRecognition();
}else{
  isMicMuted = true;
  micButton.innerHTML = micMuted;
  micButton.classList.remove('btn-outline-danger');
  micButton.classList.add('btn-primary');
  micButton.setAttribute('aria-pressed',false);
  s2t.stopSpeechRecognition();
}
}

// Fetch available models and populate the dropdown
async function populateModels() {
document.getElementById('send-button').addEventListener('click', submitRequest);

if (s2t.isSpeechSupported()){
  document.getElementById('mic-button').addEventListener('click', micRequest);
}else{
  speechDisable();
}

try {
  const data = await getAvailableModels();

  const selectElement = document.getElementById('model-select');

  data.models.forEach((model) => {
    const option = document.createElement('option');
    option.value = model.name;
    option.innerText = model.name;
    selectElement.appendChild(option);
  });

  // select option present in url parameter if present
  const queryParams = new URLSearchParams(window.location.search);
  const requestedModel = queryParams.get('model');
  // update the selection based on if requestedModel is a value in options
  if ([...selectElement.options].map(o => o.value).includes(requestedModel)) {
    selectElement.value = requestedModel;
  }
  // otherwise set to the first element if exists and update URL accordingly
  else if (selectElement.options.length) {
    selectElement.value = selectElement.options[0].value;
    //updateModelInQueryString(selectElement.value);
  }
}
catch (error) {

  err = "Unable to communitcate with AI due to the following error "+error.message;
  showNotiMsg("Communication Error", "Model Population Error", err);
}
}

function showNotiMsg(msgTitle, subT, msg){
document.getElementById('notificationModalLabel').innerHTML = DOMPurify.sanitize(msgTitle);
document.getElementById('subTitle').innerHTML = DOMPurify.sanitize(subT);
document.getElementById('notiText').innerHTML = DOMPurify.sanitize(msg);

let modal = new bootstrap.Modal(document.getElementById('notiModal'));
modal.show();
}

//UI Functions
// adjusts the padding at the bottom of scrollWrapper to be the height of the input box
function adjustPadding() {
  const inputBoxHeight = document.getElementById('input-area').offsetHeight;
  const scrollWrapper = document.getElementById('scroll-wrapper');
  scrollWrapper.style.paddingBottom = `${inputBoxHeight + 15}px`;
}

// sets up padding resize whenever input box has its height changed
const autoResizePadding = new ResizeObserver(() => {
  adjustPadding();
});
autoResizePadding.observe(document.getElementById('input-area'));


// variables to handle auto-scroll
// we only need one ResizeObserver and isAutoScrollOn variable globally
// no need to make a new one for every time submitRequest is called
const scrollWrapper = document.getElementById('scroll-wrapper');
let isAutoScrollOn = true;
// autoscroll when new line is added
const autoScroller = new ResizeObserver(() => {
  if (isAutoScrollOn) {
    scrollWrapper.scrollIntoView({behavior: "smooth", block: "end"});
  }
});

// event listener for scrolling
let lastKnownScrollPosition = 0;
let ticking = false;
document.addEventListener("scroll", (event) => {
  // if user has scrolled up and autoScroll is on we turn it off
  if (!ticking && isAutoScrollOn && window.scrollY < lastKnownScrollPosition) {
    window.requestAnimationFrame(() => {
      isAutoScrollOn = false;
      ticking = false;
    });
    ticking = true;
  }
  // if user has scrolled nearly all the way down and autoScroll is disabled, re-enable
  else if (!ticking && !isAutoScrollOn &&
    window.scrollY > lastKnownScrollPosition && // make sure scroll direction is down
    window.scrollY >= document.documentElement.scrollHeight - window.innerHeight - 30 // add 30px of space--no need to scroll all the way down, just most of the way
  ) {
    window.requestAnimationFrame(() => {
      isAutoScrollOn = true;
      ticking = false;
    });
    ticking = true;
  }
  lastKnownScrollPosition = window.scrollY;
});

function scrollToElement(elementToScroll2){
  const elementToScrollTo = elementToScroll2; //document.getElementById(elemendID);
  const topOffset = elementToScrollTo.offsetTop;
  window.scrollTo({ top: topOffset, behavior: 'smooth' });
}

function autoGrow(element) {
  const maxHeight = 500;  // This should match the max-height set in CSS

  // Temporarily reset the height to auto to get the actual scrollHeight
  $(element).css("height", "auto");
  let newHeight = element.scrollHeight;
  if (newHeight > maxHeight) {
      newHeight = maxHeight;
  }

  $(element).css("height", newHeight + "px");
}