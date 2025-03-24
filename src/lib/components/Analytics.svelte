<script>
  import { onMount } from 'svelte';
  import { fetchSessions } from '../stores/sessions';
  import Chart from 'chart.js/auto';
  
  let timeRange = 'week';
  let sessions = [];
  let pieChartCanvas;
  let barChartCanvas;
  let pieChart = null;
  let barChart = null;
  
  // Get date range
  function getDateRange(range) {
    const now = new Date();
    const endDate = new Date(now);
    endDate.setHours(23, 59, 59, 999);
    
    let startDate;
    
    if (range === 'week') {
      startDate = new Date(now);
      startDate.setDate(now.getDate() - 7);
      startDate.setHours(0, 0, 0, 0);
    } else if (range === 'month') {
      startDate = new Date(now);
      startDate.setMonth(now.getMonth() - 1);
      startDate.setHours(0, 0, 0, 0);
    } else { // year
      startDate = new Date(now);
      startDate.setFullYear(now.getFullYear() - 1);
      startDate.setHours(0, 0, 0, 0);
    }
    
    return [
      Math.floor(startDate.getTime() / 1000),
      Math.floor(endDate.getTime() / 1000)
    ];
  }
  
  $: loadData(timeRange);
  
  async function loadData(range) {
    const [startTimestamp, endTimestamp] = getDateRange(range);
    sessions = await fetchSessions(startTimestamp, endTimestamp);
    
    updateCharts();
  }
  
  function updateCharts() {
    if (sessions.length === 0) {
      destroyCharts();
      return;
    }
    
    const categoryData = processDataByCategory();
    const dayData = processDataByDay();
    
    renderPieChart(categoryData);
    renderBarChart(dayData);
  }
  
  function processDataByCategory() {
    const categories = {};
    
    sessions.forEach(session => {
      const duration = session.end_time - session.start_time;
      if (categories[session.category]) {
        categories[session.category] += duration;
      } else {
        categories[session.category] = duration;
      }
    });
    
    return categories;
  }
  
  function processDataByDay() {
    const days = {};
    
    sessions.forEach(session => {
      const date = new Date(session.start_time * 1000);
      const dateStr = date.toLocaleDateString('en-US', { weekday: 'short' });
      const duration = session.end_time - session.start_time;
      
      if (days[dateStr]) {
        days[dateStr] += duration;
      } else {
        days[dateStr] = duration;
      }
    });
    
    return days;
  }
  
  function renderPieChart(categoryData) {
    const labels = Object.keys(categoryData);
    const data = Object.values(categoryData).map(seconds => seconds / 60); // Convert to minutes
    
    const colors = [
      '#3498db', // Work
      '#9b59b6', // Study
      '#2ecc71', // Health
      '#e67e22', // Personal
      '#95a5a6', // Other
      '#f1c40f',
      '#1abc9c',
      '#e74c3c',
    ];
    
    destroyChart(pieChart);
    
    pieChart = new Chart(pieChartCanvas, {
      type: 'pie',
      data: {
        labels,
        datasets: [{
          data,
          backgroundColor: colors.slice(0, labels.length),
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            position: window.innerWidth < 768 ? 'bottom' : 'right',
          },
          tooltip: {
            callbacks: {
              label: (context) => {
                const value = context.raw;
                return `${context.label}: ${Math.round(value)} minutes`;
              }
            }
          }
        }
      }
    });
  }
  
  function renderBarChart(dayData) {
    const labels = Object.keys(dayData);
    const data = Object.values(dayData).map(seconds => seconds / 3600); // Convert to hours
    
    destroyChart(barChart);
    
    barChart = new Chart(barChartCanvas, {
      type: 'bar',
      data: {
        labels,
        datasets: [{
          label: 'Hours',
          data,
          backgroundColor: '#3498db',
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
          y: {
            beginAtZero: true,
            title: {
              display: true,
              text: 'Hours'
            }
          },
          x: {
            title: {
              display: true,
              text: 'Day'
            }
          }
        }
      }
    });
  }
  
  function destroyChart(chart) {
    if (chart) {
      chart.destroy();
    }
  }
  
  function destroyCharts() {
    destroyChart(pieChart);
    destroyChart(barChart);
    pieChart = null;
    barChart = null;
  }
  
  onMount(() => {
    loadData(timeRange);
    
    const handleResize = () => {
      if (pieChart) {
        pieChart.options.plugins.legend.position = window.innerWidth < 768 ? 'bottom' : 'right';
        pieChart.update();
      }
    };
    
    window.addEventListener('resize', handleResize);
    
    return () => {
      destroyCharts();
      window.removeEventListener('resize', handleResize);
    };
  });
</script>

<!-- Rest of the component stays the same -->
<div class="analytics-container">
  <div class="time-range-selector">
    <h2>Analytics</h2>
    <div class="range-buttons">
      <button 
        class:active={timeRange === 'week'} 
        on:click={() => timeRange = 'week'}
      >
        Week
      </button>
      <button 
        class:active={timeRange === 'month'} 
        on:click={() => timeRange = 'month'}
      >
        Month
      </button>
      <button 
        class:active={timeRange === 'year'} 
        on:click={() => timeRange = 'year'}
      >
        Year
      </button>
    </div>
  </div>
  
  {#if sessions.length === 0}
    <div class="no-data">
      <p>No data available for the selected time range</p>
    </div>
  {:else}
    <div class="charts">
      <div class="chart-container">
        <h3>Time by Category</h3>
        <div class="chart-wrapper">
          <canvas bind:this={pieChartCanvas}></canvas>
        </div>
      </div>
      
      <div class="chart-container">
        <h3>Time by Day</h3>
        <div class="chart-wrapper">
          <canvas bind:this={barChartCanvas}></canvas>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .analytics-container {
    width: 100%;
    max-width: 1000px;
    margin: 0 auto;
    padding: 1rem;
  }
  
  .time-range-selector {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    flex-wrap: wrap;
    gap: 1rem;
  }
  
  .time-range-selector h2 {
    margin: 0;
  }
  
  .range-buttons {
    display: flex;
    gap: 0.5rem;
  }
  
  .range-buttons button {
    padding: 0.5rem 1rem;
    border: 1px solid #ddd;
    background: none;
    border-radius: 0.25rem;
    cursor: pointer;
  }
  
  .range-buttons button.active {
    background-color: #3498db;
    color: white;
    border-color: #3498db;
  }
  
  .no-data {
    text-align: center;
    padding: 4rem 1rem;
    background-color: #f5f5f5;
    border-radius: 0.5rem;
    color: #777;
  }
  
  .charts {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
  }
  
  .chart-container {
    background-color: white;
    border-radius: 0.5rem;
    padding: 1rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  }
  
  .chart-container h3 {
    margin-top: 0;
    margin-bottom: 1rem;
    text-align: center;
  }
  
  .chart-wrapper {
    height: 300px;
    position: relative;
  }
  
  /* Mobile optimizations */
  @media (max-width: 768px) {
    .time-range-selector {
      flex-direction: column;
      align-items: flex-start;
    }
    
    .chart-wrapper {
      height: 250px;
    }
    
    .range-buttons button {
      padding: 0.4rem 0.8rem;
      font-size: 0.9rem;
    }
  }
</style>
