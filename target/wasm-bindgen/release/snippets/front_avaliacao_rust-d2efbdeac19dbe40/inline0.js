
export function renderLsiChart(canvasId, labels, meta, single, triple, crossover, six) {
    const ctx = document.getElementById(canvasId);
    if (!ctx) return;
    if (ctx._chart) { ctx._chart.destroy(); }
    ctx._chart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                { label: 'Meta (90%)', data: meta, borderColor: '#e74c3c', borderDash: [5,5], pointRadius: 0, fill: false },
                { label: 'Single Hop', data: single, borderColor: '#3498db', fill: false },
                { label: 'Triple Hop', data: triple, borderColor: '#2ecc71', fill: false },
                { label: 'Crossover', data: crossover, borderColor: '#e67e22', fill: false },
                { label: '6 Metros', data: six, borderColor: '#9b59b6', fill: false }
            ]
        },
        options: {
            responsive: true,
            plugins: { legend: { position: 'bottom' } },
            scales: { y: { min: 0, max: 100, ticks: { callback: v => v + '%' } } }
        }
    });
}
