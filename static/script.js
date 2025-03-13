document.addEventListener('DOMContentLoaded', function() {
    // Mobile menu toggle
    const menuToggle = document.querySelector('.mobile-menu-toggle');
    const mobileMenu = document.querySelector('.mobile-menu');

    menuToggle.addEventListener('click', function() {
        mobileMenu.classList.toggle('active');

        // Animate hamburger to X
        const spans = menuToggle.querySelectorAll('span');
        if (mobileMenu.classList.contains('active')) {
            spans[0].style.transform = 'rotate(45deg) translate(5px, 5px)';
            spans[1].style.opacity = '0';
            spans[2].style.transform = 'rotate(-45deg) translate(5px, -5px)';
        } else {
            spans[0].style.transform = 'none';
            spans[1].style.opacity = '1';
            spans[2].style.transform = 'none';
        }
    });

    // Close mobile menu when clicking outside
    document.addEventListener('click', function(event) {
        if (!menuToggle.contains(event.target) && !mobileMenu.contains(event.target) && mobileMenu.classList.contains('active')) {
            mobileMenu.classList.remove('active');
            const spans = menuToggle.querySelectorAll('span');
            spans[0].style.transform = 'none';
            spans[1].style.opacity = '1';
            spans[2].style.transform = 'none';
        }
    });

    // Account card hover effect
    const accountCards = document.querySelectorAll('.account-card');
    accountCards.forEach(card => {
        card.addEventListener('mouseenter', function() {
            accountCards.forEach(c => c !== card && c.style.setProperty('transform', 'scale(0.98)'));
        });
        card.addEventListener('mouseleave', function() {
            accountCards.forEach(c => c.style.removeProperty('transform'));
        });
    });

    // Bar chart animation
    const bars = document.querySelectorAll('.bar');
    setTimeout(() => {
        bars.forEach(bar => {
            const height = bar.style.height;
            bar.style.height = '0%';
            setTimeout(() => {
                bar.style.height = height;
            }, 100);
        });
    }, 500);

    // Notification badge pulse animation
    const notificationBtn = document.querySelector('.notification-btn');
    if (notificationBtn) {
        setInterval(() => {
            notificationBtn.classList.add('pulse');
            setTimeout(() => {
                notificationBtn.classList.remove('pulse');
            }, 1000);
        }, 5000);
    }

    // Add pulse animation style
    const style = document.createElement('style');
    style.textContent = `
        @keyframes pulse {
            0% {
                transform: scale(1);
            }
            50% {
                transform: scale(1.1);
            }
            100% {
                transform: scale(1);
            }
        }
        .pulse {
            animation: pulse 1s ease-in-out;
        }
    `;
    document.head.appendChild(style);
});
