<script lang="ts">
  const slides = [
    {
      value: '89,400',
      title: 'Total Followers',
      desc: '21% more than last month',
      descClass: 'text-success',
    },
    {
      value: '18',
      title: 'Posts Today Cross-Platform',
      desc: 'Keep it up!',
      descClass: 'text-success',
    },
    {
      value: '265K',
      title: 'Total Engagements',
      desc: '83% more than last week',
      descClass: 'text-success',
    },
  ];

  let index = $state(0);

  $effect(() => {
    const id = setInterval(() => {
      index = (index + 1) % slides.length;
    }, 8000);
    return () => clearInterval(id);
  });
</script>

<div class="card bg-neutral w-96 shadow-xl">
  <div class="card-body text-primary">
    <div class="flex flex-row space-x-2 font-mono">
      <span class="badge badge-outline badge-sm badge-secondary">SOCIALS</span>
      <span class="badge badge-outline badge-sm badge-secondary">ATTRACTION</span>
    </div>

    {#each slides as slide, i (slide.title)}
      {#if i === index}
        <div class="stat">
          <div class="stat-figure">
            <div class="avatar avatar-online">
              <div class="w-16 rounded-full">
                <img
                  alt=""
                  src="https://img.daisyui.com/images/profile/demo/anakeen@192.webp"
                />
              </div>
            </div>
          </div>
          <div class="stat-value">{slide.value}</div>
          <div class="stat-title text-primary">{slide.title}</div>
          <div class="stat-desc {slide.descClass}">{slide.desc}</div>
        </div>
      {/if}
    {/each}

    <div class="flex justify-center gap-2 mt-2">
      {#each slides as _, i (i)}
        <button
          type="button"
          class="btn btn-xs btn-circle {i === index ? 'btn-primary' : 'btn-ghost'}"
          aria-label="Show slide {i + 1}"
          onclick={() => (index = i)}
        ></button>
      {/each}
    </div>
  </div>
</div>
