<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/state';
  import {
    getStripePlans,
    createCheckoutSession,
    getPayments,
    formatMoney,
    type StripePlan,
    type Payment,
  } from '$lib/api';
  import { toastStore } from '$lib/stores/toast';

  let plans = $state<StripePlan[]>([]);
  let configured = $state(true);
  let payments = $state<Payment[]>([]);
  let loading = $state(true);
  let buying = $state<string | null>(null);

  onMount(async () => {
    const checkout = page.url.searchParams.get('checkout');
    if (checkout === 'success') {
      toastStore.success('Thanks! Your payment is being confirmed.');
    } else if (checkout === 'cancelled') {
      toastStore.info('Checkout cancelled. You have not been charged.');
    }

    await Promise.all([loadPlans(), loadPayments()]);
    loading = false;

    // Stripe confirms a payment by calling our webhook, which can land a moment
    // after the redirect back here. Re-check briefly so the receipt shows up.
    if (checkout === 'success') {
      for (let attempt = 0; attempt < 5; attempt++) {
        await new Promise((resolve) => setTimeout(resolve, 2000));
        await loadPayments();
      }
    }
  });

  async function loadPlans() {
    try {
      const result = await getStripePlans();
      plans = result.plans;
      configured = result.configured;
    } catch (e) {
      toastStore.error('Failed to load plans');
    }
  }

  async function loadPayments() {
    try {
      payments = await getPayments();
    } catch (e) {
      toastStore.error('Failed to load payments');
    }
  }

  async function buy(plan: StripePlan) {
    buying = plan.id;
    try {
      const url = await createCheckoutSession(plan.id);
      window.location.assign(url);
    } catch (e) {
      toastStore.error('Could not start checkout');
      buying = null;
    }
  }
</script>

<div class="max-w-3xl mx-auto px-4 py-10">

  <div class="mb-8">
    <h1 class="text-4xl font-bold">Payments</h1>
    <p class="text-base-content/60 mt-1">
      Demo Stripe Checkout (<code>POST /stripe/checkout</code>). Payments are recorded by a signed webhook
      (<code>POST /stripe/webhook</code>). In test mode, pay with card <code>4242 4242 4242 4242</code>.
    </p>
  </div>

  {#if !configured}
    <div class="alert alert-warning mb-8">
      <span>
        Stripe isn't configured on the server yet. Set <code>STRIPE_SECRET_KEY</code> and
        <code>STRIPE_WEBHOOK_SECRET</code> to try the demo.
      </span>
    </div>
  {/if}

  <!-- Plans -->
  <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-10">
    {#each plans as plan (plan.id)}
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h2 class="card-title">{plan.name}</h2>
          <p class="text-3xl font-bold text-primary">{formatMoney(plan.amount, plan.currency)}</p>
          <p class="text-base-content/60 text-sm">{plan.description}</p>
          <div class="card-actions mt-2">
            <button
              type="button"
              class="btn btn-primary w-full"
              disabled={!configured || buying !== null}
              onclick={() => buy(plan)}
            >
              {#if buying === plan.id}
                <span class="loading loading-spinner loading-sm"></span> Redirecting…
              {:else}
                Buy
              {/if}
            </button>
          </div>
        </div>
      </div>
    {/each}
  </div>

  <!-- Payment history -->
  <h2 class="text-2xl font-bold mb-4">Your payments</h2>
  {#if loading}
    <div class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else if payments.length === 0}
    <div class="text-center py-12 text-base-content/40">
      <div class="text-6xl mb-4">🧾</div>
      <p class="text-lg">No payments yet. Buy a plan above.</p>
    </div>
  {:else}
    <div class="space-y-3">
      {#each payments as payment (payment.id)}
        <div class="card bg-base-100 shadow-md">
          <div class="card-body py-4 flex-row items-center justify-between gap-4">
            <div class="min-w-0">
              <p class="font-semibold leading-tight">{payment.plan_name}</p>
              <p class="text-base-content/40 text-xs mt-1">
                {new Date(payment.created_at).toLocaleString()}
              </p>
            </div>
            <div class="flex items-center gap-3 shrink-0">
              <span class="badge badge-success badge-sm capitalize">{payment.status}</span>
              <span class="font-bold">{formatMoney(payment.amount, payment.currency)}</span>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}

</div>
