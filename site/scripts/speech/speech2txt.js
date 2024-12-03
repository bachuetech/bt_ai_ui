class SpeechRecognizer {
    sRecognizer;
    preText;
    textDestinationElement;
    callbackFunction;
    manualStop = true;

    constructor(textDestinationID, cbf) {
        this.callbackFunction = cbf;

        this.textDestinationElement = document.getElementById(textDestinationID);

        if ('SpeechRecognition' in window || 'webkitSpeechRecognition' in window) {
            this.sRecognizer = new (window.webkitSpeechRecognition || window.SpeechRecognition)();
            if (this.sRecognizer) {
                this.isSpeechReconSupported = true;
                this.sRecognizer.lang = 'en-US'; // Set the language
                this.sRecognizer.continuous = false; //true;
                this.sRecognizer.interimResults = true; // Set to true for interim results
                this.sRecognizer.minimumSpeechDuration = 5000;

                this.sRecognizer.onresult = (event) => {
                    console.log("onResult");
                    let transcriptInterim = event.results[0][0].transcript; // Get the recognized text
                    console.log("onResult: " + transcriptInterim);
                    if (this.preText) {
                        this.textDestinationElement.value = this.preText + " " + transcriptInterim; // Insert the text into the input
                    } else {
                        this.textDestinationElement.value = transcriptInterim;
                    }
                };

                this.sRecognizer.onerror = function (event) {
                    console.error('Error occurred in recognition: ' + event.error);
                };

                this.sRecognizer.onspeechend = () => {
                    console.log("Speech has stopped being detected");
                };

                this.sRecognizer.onsoundend = function () {
                    console.log('Sound Stop.');
                };

                this.sRecognizer.onaudioend = function () {
                    console.log('Finished capturing audio.');

                };

                this.sRecognizer.onend = cbf; 
                console.log('Voice recognition turned off.');
                if (!this.manualStop) {
                    console.log('Ready for Autostop.');
                    this.autoStop();
                }
            }else{
                this.isSpeechReconSupported = false;
                console.log('Speech Recognition API not supported');
            }
        } else {
            this.isSpeechReconSupported = false;
            console.log('Speech Recognition API not supported');
        }
    }


    autoStop(){
        console.log('Autostop called');
        if (!this.manualStop){
            console.log(typeof this.callbackFunction);
            callbackFunction();
        }
    }

    //Function to return if Speech Recognition is supported
    isSpeechSupported(){
        return this.isSpeechReconSupported;
    }

    startSpeechRecognition(){
        this.manualStop = false;
        this.preText = this.textDestinationElement.value;
        this.sRecognizer.start();
    }

    stopSpeechRecognition() {
        if (this.sRecognizer) {
            this.manualStop = true;
            this.preText = null;
            this.sRecognizer.stop();
        }
    }

}