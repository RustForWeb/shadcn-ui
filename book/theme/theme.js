window.addEventListener('message', (event) => {
    if (!event.data.mdbookTrunk) {
        return;
    }

    const data = event.data.mdbookTrunk;
    const iframe = Array.from(document.getElementsByTagName('iframe')).find(
        (iframe) => iframe.contentWindow === event.source
    );
    if (iframe) {
        iframe.style.height = `${data.height}px`;
    }
});
