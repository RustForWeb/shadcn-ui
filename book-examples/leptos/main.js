document.addEventListener('DOMContentLoaded', () => {
    const resizeObserver = new ResizeObserver(() => {
        if (window.top) {
            window.top.postMessage({
                mdbookTrunk: {
                    width: document.body.scrollWidth,
                    height: document.body.scrollHeight
                }
            });
        }
    });
    resizeObserver.observe(document.body);
});
