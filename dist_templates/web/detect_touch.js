(function (w) {
    w.__is_touch_available__ = function detect_touch() {
        return (('ontouchstart' in window) ||
            (navigator.maxTouchPoints > 0) ||
            (navigator.msMaxTouchPoints > 0));
    }
})(window);