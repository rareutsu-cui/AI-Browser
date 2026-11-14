// 見た目だけを変更する。スクロール位置や入力イベントには触れない。
(() => {
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
})();
