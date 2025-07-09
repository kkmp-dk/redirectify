// ==UserScript==
// @name         Open Links with System Default Handler for 'redirect'
// @version      0.1
// @description  Open links using the custom redirect protocol. using Ctrl + Shift + Left Click.
// @author       kkmp
// @match        *://*/*
// @grant        none
// ==/UserScript==


(function () {
  'use strict';

  // Listen for mouse clicks on the document
  document.addEventListener('click', function (event) {
    // Check if Ctrl + Shift are pressed and it's a left click
    if (event.ctrlKey && event.shiftKey && event.button === 0) {
      // Prevent the default browser behavior
      event.preventDefault();

      // Find the clicked element
      let target = event.target;

      // Traverse up the DOM tree to find the nearest <a> tag
      while (target && target.tagName !== 'A') {
        target = target.parentElement;
      }

      // If a link is found, process it
      if (target && target.href) {
        const link = target.href;

        // Use the custom redirect protocol
        const redirectLink = `redirect:${link}`;
        //window.open(redirectLink, '_blank');
        GM_openInTab(redirectLink);

        // Log the action for debugging
        console.log(`Opening link with redirect protocol: ${redirectLink}`);
      }
    }
  });
})();
