/**
 * Change active file of files.
 *
 * @param {Element} container
 * @param {string | null} name
 */
const changeTrunkFile = (container, name) => {
    for (const child of container.children) {
        if (!(child instanceof HTMLElement)) {
            continue;
        }

        if (child.classList.contains('mdbook-trunk-files')) {
            for (const file of child.children) {
                if (!(file instanceof HTMLElement)) {
                    continue;
                }

                if (file.dataset.file === name) {
                    file.classList.add('active');
                } else {
                    file.classList.remove('active');
                }
            }
        } else if (child.classList.contains('mdbook-trunk-file-content')) {
            if (child.dataset.file === name) {
                child.classList.remove('hidden');
            } else {
                child.classList.add('hidden');
            }
        }
    }
};

document.addEventListener('DOMContentLoaded', () => {
    const files = document.querySelectorAll('.mdbook-trunk-file');
    for (const file of files) {
        file.addEventListener('click', () => {
            if (!(file instanceof HTMLElement)) {
                return;
            }

            if (!file.parentElement || !file.parentElement.parentElement) {
                return;
            }

            const container = file.parentElement.parentElement;
            const name = file.dataset.file;

            changeTrunkFile(container, file.classList.contains('active') ? null : name);
        });
    }
});
