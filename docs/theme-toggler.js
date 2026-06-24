(() => {
	"use strict";
	const hljsStyleCdnDefault =
		"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/default.min.css";
	const hljsStyleCdnDark =
		"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/base16/synth-midnight-terminal-dark.min.css";

	const hlCssLinkTag = document.querySelector("#hl-css");
	const storedTheme = localStorage.getItem("theme");
	const cdnMap = {
		light: hljsStyleCdnDefault,
		dark: hljsStyleCdnDark,
	};
	const getPreferredTheme = () => {
		if (storedTheme) {
			return storedTheme;
		}
		return window.matchMedia("(prefers-color-scheme: dark)").matches
			? "dark"
			: "light";
	};
	const setTheme = function (theme) {
		if (
			theme === "auto" &&
			window.matchMedia("(prefers-color-scheme: dark)").matches
		) {
			document.documentElement.setAttribute("data-bs-theme", "dark");
		} else {
			document.documentElement.setAttribute("data-bs-theme", theme);
		}
	};
	const getToggleToTheme = (currentTheme) =>
		currentTheme === "dark" ? "light" : "dark";
	const initTheme = () => {
		const currentTheme = getPreferredTheme();
		setTheme(currentTheme);
		const toggleTargetTheme = getToggleToTheme(currentTheme);
		hlCssLinkTag.setAttribute("href", cdnMap?.[currentTheme]);
		document.querySelectorAll("[data-bs-theme-value]").forEach((toggle) => {
			const toggleTheme = () => {
				document
					.querySelectorAll("[data-bs-theme-value]")
					.forEach((_toggle) => {
						const targetTheme = getToggleToTheme(
							document.documentElement.getAttribute("data-bs-theme"),
						);
						hlCssLinkTag.setAttribute("href", cdnMap?.[targetTheme]);
						setTheme(targetTheme);
						localStorage.setItem("theme", targetTheme);
					});
			};
			console.log("", toggleTargetTheme);
			toggle.setAttribute("data-bs-theme-value", toggleTargetTheme);
			toggle.addEventListener("click", toggleTheme);
		});
	};
	initTheme();
	window
		.matchMedia("(prefers-color-scheme: dark)")
		.addEventListener("change", () => {
			if (storedTheme !== "light" || storedTheme !== "dark") {
				setTheme(getPreferredTheme());
			}
		});

	window.rustQuizSetTheme = setTheme;
})();
