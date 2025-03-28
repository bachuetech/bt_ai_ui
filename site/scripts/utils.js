function splitString(input, delimiter = ":") {
    // Use a regular expression to match the first occurrence of the delimiter and split
    let regex = new RegExp(`^(.*?)(?:${delimiter})(.*)$`);
    let match = input.match(regex);
    
    if (match) {
        // Return the two parts (before and after the first delimiter)
        return { var1: match[1], var2: match[2] };
    } else {
        // If no delimiter is found, return the input as the first part, with an empty second part
        return { var1: input, var2: '' };
    }
}

function fixLatexIssue(text){
    const regex = /\\text\{(.*?)\}/g;
    return text.replace(regex, (match, content) => {
      return match.replace(/%/g, '\\%');
    });
  }
  
  //It seems that some characters are re-rendered. e.g. &gt; becomes &amp;gt;
  function cleanHTMLEntities(txtToClean){
    return txtToClean.replace(/&amp;/g,"&").replace(/&lt;/g,"<").replace(/&gt;/g,">").replace(/&quot;/g,"\"").replace(/&apos;/g,"\'");
  }
  
  //Function to build the TAG for Google Prettify
  function buildLanguangeCodeSection(text) {
    var codeSection = '<pre class="prettyprint linenums">'+text.trim();
    const match = text.match(/-\w+/);
    if (match) {
      const word = match[0].substring(1); // Remove the dash
      codeSection = codeSection+"<h4>"+word.toUpperCase()+"</h4>";
    }
    return codeSection;
  }
  
  //Function to add the pre tag for Google Prettify
  function prettifyCode(text) {
    return text.replace(/<code class="([^"]+)">/g, buildLanguangeCodeSection)
               .replace(/<\/code>/g, '</code></pre>');
  }
  
//Format all numbers:
function formatNumbers(text) {
      const regex = /\d+(?:\.\d+)?(?![^<]*>)/g;
    return text.replace(regex, match => {
      const [wholeNumber, decimalPart] = match.split('.');
      const formattedNumber = parseInt(wholeNumber).toLocaleString('en-US');
      if (decimalPart) {
        return `${formattedNumber}.${decimalPart}`;
      }
      return formattedNumber;
    });
  }

  function DamageControlToHTMLCleansing(text) {
    // Convert \frac{numerator}{denominator} to <math><mfrac><mn>numerator</mn><mn>denominator</mn></mfrac></math>
    text = text.replace(/\\frac{([^}]+)}{([^}]+)}/g, '<math><mfrac><mn>$1</mn><mn>$2</mn></mfrac></math>');

	  // Convert \boxed{expression} to <span style="border: 1px solid black;">expression</span>
    text = text.replace(/\$\\boxed{([^}]+)}\$/g, '<span style="border: 1px solid black;">$1</span>');    

    // Convert \boxed{expression} to <span style="border: 1px solid black;">expression</span>
    text = text.replace(/\\boxed{([^}]+)}/g, '<span style="border: 1px solid black;">$1</span>');
  
    // Convert \[a-zA-Z]+ to <span style="font-style: italic;">expression</span>  /\\([^\\s\n]+)/g
    text.replace(/\\text\{([^}]+)}/g, '<span style="font-style: italic;">$1</span>');

    return text;
  }

  function getCurrentDate() {
    const date = new Date();
    const year = date.getFullYear();
    const monthNames = ["January",
      "February",
      "March",
      "April",
      "May",
      "June",
      "July",
      "August",
      "September",
      "October",
      "November",
      "December"];
    const month = monthNames[date.getMonth()];
    const day = date.getDate().toString().padStart(2, '0'); // Pad day with leading zero if needed
  
    return `${month} ${day}, ${year}`;
  }
  
  function getCurrentTime() {
   const now = new Date(); // Get the current date and time
  
   // Format the time
   const hours = String(now.getHours()).padStart(2, '0'); // Get hours
   const minutes = String(now.getMinutes()).padStart(2, '0'); // Get minutes
   //const seconds = String(now.getSeconds()).padStart(2, '0'); // Get seconds
  
    return `${hours}:${minutes}`;
  }