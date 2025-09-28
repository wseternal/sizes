pub type Property<'a> = &'a str;
pub type PropertyPrefix<'a> = &'a str;

pub const KStats                            :Property = "stats";
pub const KSSTables                         :Property = "sstables";
pub const KLevelStats                       :Property = "levelstats";
pub const KEstimateNumKeys                  :Property = "estimate-num-keys";
pub const KBackgroundErrors                 :Property = "background-errors";
pub const KEstimateLiveDataSize             :Property = "estimate-live-data-size";
pub const KNumSnapshots                     :Property = "num-snapshots";
pub const KOldestSnapshotTime               :Property = "oldest-snapshot-time";
pub const KNumLiveVersions                  :Property = "num-live-versions";
pub const KCurrentSuperVersionNumber        :Property = "current-super-version-number";
pub const KTotalSstFilesSize                :Property = "total-sst-files-size";
pub const KAggregatedTableProperties        :Property = "aggregated-table-properties";
pub const KNumFilesAtLevelPrefix            :PropertyPrefix = "num-files-at-level";
pub const KCompressionRatioAtLevelPrefix    :PropertyPrefix = "compression-ratio-at-level";
pub const KAggregatedTablePropertiesAtLevel :PropertyPrefix = "aggregated-table-properties-at-level";