//Fetch the list of voices and populate the voice options.
function loadVoices() {
    // Fetch the available voices.
      var voices = speechSynthesis.getVoices();
  };
  
  // Execute loadVoices.
  loadVoices();
  
  //This is needed to change the voice, not sure how to change the voice since the begining!
  window.speechSynthesis.onvoiceschanged = function(e) {
      loadVoices();
  };
  
  var txt2Speech = function() {
      this.currentRate  = 1;
      this.currentVoice = 3;
      this.defaultRate  = 1.4;
      this.defaultVoice = 3;
  };
  
  txt2Speech.prototype.rateDown = function(){
      if (this.currentRate > 0.1){
          this.currentRate = this.currentRate - 0.1;
      }
  };
  
  txt2Speech.prototype.rateUp = function(){
      if (this.currentRate < 1.5){
          this.currentRate = this.currentRate + 0.1;
      }
  };
  
  txt2Speech.prototype.resetRate = function(){
      this.currentRate = 1;
  };
  
  txt2Speech.prototype.speak = function(message){
      this.readAloud(message,this.defaultRate,this.defaultVoice);
  };
  
  txt2Speech.prototype.readAloud =  function(messageToRead, rate, voiceID){
      var utterance  = new SpeechSynthesisUtterance();
      const userAgent = navigator.userAgent;
      if (userAgent.indexOf("Firefox") !== -1 || userAgent.indexOf("Opera") !== -1) {
          utterance.voice = speechSynthesis.getVoices().filter(function(voice) { return voice.name === 'Microsoft David - English (United States)'; })[0];
          utterance.voiceURI = 'Microsoft David - English (United States)';
      }else{
          utterance.voice = speechSynthesis.getVoices().filter(function(voice) { return voice.name === 'Google US English'; })[0];
          utterance.voiceURI = 'Google US English';
      }
      utterance.volume = 1; // 0 to 1
      utterance.pitch = 1;
      utterance.rate = 0.9; // 0.1 to 10
      utterance.text = messageToRead;
      utterance.lang = 'en-US';
      speechSynthesis.speak(utterance);
  };