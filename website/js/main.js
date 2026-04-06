document.addEventListener("DOMContentLoaded", () => {
    const navbar = document.querySelector(".navbar");
    const revealCards = document.querySelectorAll(".reveal-card");
    const statValues = document.querySelectorAll(".stat-value");
    const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    const applyNavbarState = () => {
        if (!navbar) return;
        if (window.scrollY > 20) {
            navbar.classList.add("scrolled");
        } else {
            navbar.classList.remove("scrolled");
        }
    };

    applyNavbarState();
    window.addEventListener("scroll", applyNavbarState);

    document.querySelectorAll('a[href^="#"]').forEach((anchor) => {
        anchor.addEventListener("click", (event) => {
            const href = anchor.getAttribute("href");
            if (!href || href === "#") return;
            const target = document.querySelector(href);
            if (!target) return;
            event.preventDefault();
            target.scrollIntoView({ behavior: "smooth", block: "start" });
        });
    });

    if (!("IntersectionObserver" in window) || reducedMotion) {
        revealCards.forEach((card) => card.classList.add("revealed"));
    } else {
        const revealObserver = new IntersectionObserver(
            (entries) => {
                entries.forEach((entry) => {
                    if (!entry.isIntersecting) return;
                    entry.target.classList.add("revealed");
                    revealObserver.unobserve(entry.target);
                });
            },
            { threshold: 0.12, rootMargin: "0px 0px -40px 0px" }
        );

        revealCards.forEach((card) => revealObserver.observe(card));
    }

    const animateValue = (element) => {
        const raw = Number(element.dataset.count || "0");
        if (reducedMotion) {
            const suffix = element.parentElement?.querySelector(".stat-label")?.textContent === "Uptime" ? "%" : "+";
            element.textContent = suffix === "%" ? `${raw.toFixed(2)}%` : `${Math.round(raw).toLocaleString()}+`;
            return;
        }

        const isDecimal = String(element.dataset.count || "").includes(".");
        const suffix = element.parentElement?.querySelector(".stat-label")?.textContent === "Uptime" ? "%" : "+";
        const duration = 1200;
        const start = performance.now();

        const frame = (now) => {
            const progress = Math.min((now - start) / duration, 1);
            const eased = 1 - Math.pow(1 - progress, 3);
            const value = raw * eased;

            if (isDecimal) {
                element.textContent = `${value.toFixed(2)}${suffix === "%" ? "%" : ""}`;
            } else {
                element.textContent = `${Math.round(value).toLocaleString()}${suffix === "+" ? "+" : ""}`;
            }

            if (progress < 1) {
                requestAnimationFrame(frame);
            } else if (suffix === "+") {
                element.textContent = `${Math.round(raw).toLocaleString()}+`;
            }
        };

        requestAnimationFrame(frame);
    };

    if (!("IntersectionObserver" in window)) {
        statValues.forEach((value) => animateValue(value));
        return;
    }

    const statObserver = new IntersectionObserver(
        (entries) => {
            entries.forEach((entry) => {
                if (!entry.isIntersecting) return;
                animateValue(entry.target);
                statObserver.unobserve(entry.target);
            });
        },
        { threshold: 0.35 }
    );

    statValues.forEach((value) => statObserver.observe(value));
});
