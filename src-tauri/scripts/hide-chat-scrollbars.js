// 表示だけを変更。スクロール位置・ホイール・キー入力には触れない。
(() => {
  const install = () => {
    const id = 'ai-browser-chat-scrollbars';
    if (document.getElementById(id)) return;
    const parent = document.head || document.documentElement;
    if (!parent) return;
    const style = document.createElement('style');
    style.id = id;
    style.textContent = `
      * { scrollbar-width: none !important; }
      *::-webkit-scrollbar {
        display: none !important;
        width: 0 !important;
        height: 0 !important;
      }
    `;
    parent.appendChild(style);
  };
  install();
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', install, { once: true });
  }
})();
