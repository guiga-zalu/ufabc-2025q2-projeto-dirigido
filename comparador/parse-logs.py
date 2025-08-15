from dataclasses import dataclass
import math
from pathlib import Path
from typing import Optional


@dataclass(init=False)
class Metric:
    name: str
    values: list[float]
    min: Optional[float] = None
    max: Optional[float] = None
    sum1: Optional[float] = None
    sum2: Optional[float] = None
    mean: Optional[float] = None
    var: Optional[float] = None
    unity: str = ""
    scale_into_unity: str = ""
    scale_by: float = 1
    proper_rounding: Optional[int] = None
    str_span: bool = True
    str_stddev: bool = True
    k: int = 1

    def __init__(
        self,
        name: str,
        min: Optional[float] = None,
        max: Optional[float] = None,
        sum1: Optional[float] = None,
        sum2: Optional[float] = None,
        mean: Optional[float] = None,
        var: Optional[float] = None,
        unity: str = "",
        scale_into_unity: str = "",
        scale_by: float = 1.0,
        proper_rounding: Optional[int] = None,
        k: int = 1,
        str_span: bool = True,
        str_stddev: bool = True,
    ):
        self.name = name
        self.values = []
        self.min = min
        self.max = max
        self.sum1 = sum1
        self.sum2 = sum2
        self.mean = mean
        self.var = var
        self.unity = unity
        self.scale_into_unity = scale_into_unity
        self.scale_by = scale_by
        self.proper_rounding = proper_rounding
        self.str_span = str_span
        self.str_stddev = str_stddev
        self.k = k

    def compile(self):
        if len(self.values) != 0:
            if (
                False
                or self.min is None
                and self.max is None
                and self.sum1 is None
                and self.sum2 is None
            ):
                self.min = min(self.values)
                self.max = max(self.values)
                total_len = len(self.values)
                self.sum1 = sum(self.values)
                self.sum2 = sum(x**2 for x in self.values)
                self.mean = self.sum1 / total_len
                self.var = self.sum2 / total_len - (self.mean**2)
                self.k = total_len
                self.values = []
                return

        assert self.min is not None
        assert self.max is not None
        assert self.sum1 is not None
        assert self.sum2 is not None

        total_len = self.k
        if len(self.values) != 0:
            total_len += len(self.values)
            self.min = min(self.min, min(self.values))
            self.max = max(self.max, max(self.values))
            self.sum1 += sum(self.values)
            self.sum2 += sum(x**2 for x in self.values)
            self.k = total_len
            self.values = []
        self.mean = self.sum1 / total_len
        self.var = self.sum2 / total_len - (self.mean**2)

    def __add__(self, other: "Metric") -> "Metric":
        assert self.name == other.name
        self.compile()
        other.compile()
        assert (
            self.min is not None
            and self.max is not None
            and self.sum1 is not None
            and self.sum2 is not None
        )
        assert (
            other.min is not None
            and other.max is not None
            and other.sum1 is not None
            and other.sum2 is not None
        )
        metric = Metric(
            self.name,
            min=min(self.min, other.min),
            max=max(self.max, other.max),
            sum1=self.sum1 + other.sum1,
            sum2=self.sum2 + other.sum2,
            unity=self.unity,
            scale_into_unity=self.scale_into_unity,
            scale_by=self.scale_by,
            proper_rounding=self.proper_rounding,
            k=self.k + other.k,
        )
        metric.compile()
        return metric

    def __str__(self) -> str:
        self.compile()

        name = self.name
        if self.scale_into_unity:
            name += f" ({self.scale_into_unity})"
        elif self.unity:
            name += f" ({self.unity})"

        assert self.min is not None
        assert self.max is not None
        assert self.mean is not None
        assert self.var is not None

        scale = self.scale_by or 1.0

        min_ = self.min * scale
        max_ = self.max * scale
        mean = self.mean * scale
        stddev = math.sqrt(self.var) * scale

        str_ = [name.ljust(10) + ":"]
        if self.proper_rounding is not None and stddev > 0.0:
            significant_digit = math.floor(math.log10(stddev))
            rounding_scale = self.proper_rounding - significant_digit
            min_ = round(min_, rounding_scale)
            max_ = round(max_, rounding_scale)
            mean = round(mean, rounding_scale)
            stddev = round(stddev, rounding_scale)
            if self.str_span:
                str_.append(str(min_) + " |")
            str_.append(str(mean))
            if self.str_stddev:
                str_.append(f"± {stddev}")
            if self.str_span:
                str_.append("| " + str(max_))
        else:
            if self.str_span:
                str_ += f"{min_:,.2f} |"
            str_ += f"{mean:,.2f}"
            if self.str_stddev:
                str_ += f" ± {stddev:,.2f}"
            if self.str_span:
                str_ += f"| {max_:,.2f}"
        return " ".join(str_)


@dataclass(init=False)
class Codec:
    name: str
    size_bytes: Metric
    time_mcs: Metric
    lossy: bool
    hashes: dict[str, Metric]
    metrics: dict[str, Metric]
    relative_sizes: Metric
    k: int = 1

    def __init__(
        self,
        name: str,
        size_bytes: Optional[Metric] = None,
        time_mcs: Optional[Metric] = None,
        lossy: bool = False,
        hashes: dict[str, Metric] = {},
        metrics: dict[str, Metric] = {},
        k: int = 1,
    ):
        self.name = name
        self.lossy = lossy
        self.hashes = hashes
        self.metrics = metrics
        self.k = k
        self.size_bytes = size_bytes or Metric(
            "Filesize",
            unity="B",
            scale_into_unity="KiB",
            scale_by=1e-3,
            proper_rounding=0,
            str_span=False,
        )
        self.time_mcs = time_mcs or Metric(
            "Time",
            unity="μs",
            scale_into_unity="ms",
            scale_by=1e-3,
            proper_rounding=1,
        )
        self.relative_sizes = Metric("Relative Size", unity="%", proper_rounding=1)

    def add(
        self, other: "Codec", total_self: int, total_other: int
    ) -> Optional["Codec"]:
        if self.name != other.name or self.lossy != other.lossy:
            return None

        total_len = total_self + total_other
        new = Codec(
            self.name,
            self.size_bytes + other.size_bytes,
            self.time_mcs + other.time_mcs,
            self.lossy,
            k=total_len,
        )

        if self.lossy:
            hashes = self.hashes.copy()
            for name in other.hashes.keys():
                if name not in hashes:
                    hashes[name] = other.hashes[name]
                    continue
                hashes[name] = hashes[name] + other.hashes[name]
            new.hashes = hashes

            metrics = self.metrics.copy()
            for name in other.metrics.keys():
                if name not in metrics:
                    metrics[name] = other.metrics[name]
                    continue
                metrics[name] = metrics[name] + other.metrics[name]
            new.metrics = metrics

        self.relative_sizes.compile()
        other.relative_sizes.compile()

        new.relative_sizes = self.relative_sizes + other.relative_sizes

        return new

    def compile(self):
        for metric in self.hashes.values():
            metric.compile()
        for metric in self.metrics.values():
            metric.compile()
        self.time_mcs.compile()
        self.relative_sizes.compile()
        self.size_bytes.compile()


@dataclass
class Status:
    total_files: int
    codecs: dict[str, Codec]

    def compile(self):
        for codec in self.codecs.values():
            codec.compile()

    def __add__(self, other: "Status") -> "Status":
        total_files = self.total_files + other.total_files
        codecs = self.codecs.copy()
        for name, codec in other.codecs.items():
            if name not in codecs:
                codecs[name] = codec
                continue
            add = codecs[name].add(codec, self.total_files, other.total_files)
            if add is not None:
                codecs[name] = add
        return Status(total_files, codecs)


def ensure_entry(d: dict, key, default_value):
    if key not in d:
        if isinstance(default_value, type) or callable(default_value):
            d[key] = default_value()
        else:
            d[key] = default_value
    return d[key]


def parse_log_line(
    line: str,
    stats: Status,
    codec: Optional[Codec] = None,
    ignore_zeroes: bool = False,
) -> Optional[Codec]:
    if line.startswith("Image "):
        stats.total_files += 1
        return None

    parts = line[:-1].split(",")
    descriptor = parts.pop(0)

    if descriptor == "Codec":
        name, size_bytes, time_spent_mcs, relative_size = parts
        codec = ensure_entry(
            stats.codecs,
            name,
            lambda: Codec(name, lossy="Lossy" in name),
        )
        codec.size_bytes.values.append(int(size_bytes[:-1]))
        codec.time_mcs.values.append(int(time_spent_mcs[:-3]))
        codec.relative_sizes.values.append(float(relative_size[:-1]))
        return codec

    if not codec or not codec.lossy:
        return None

    if descriptor == "Hash":
        name, value = parts
        metric = ensure_entry(
            codec.hashes, name, lambda: Metric(name, unity="%", proper_rounding=1)
        )
        value = float(value.replace("%", ""))
        if value > 0.0 or not ignore_zeroes:
            metric.values.append(value)
        return codec

    if descriptor == "Metric":
        name, value = parts
        metric = ensure_entry(
            codec.metrics, name, lambda: Metric(name, proper_rounding=1)
        )
        value = float(value)
        if value > 0.0 or not ignore_zeroes:
            metric.values.append(value)
        return codec


def parse_log_file(filename: Path, ignore_zeroes: bool) -> Status:
    stats: Status = Status(0, {})
    codec: Optional[Codec] = None
    with filename.open("r") as f:
        for line in f.readlines():
            try:
                codec = parse_log_line(line, stats, codec, ignore_zeroes)
            except ValueError:
                break
    return stats


def divide_stats(stats: Status):
    print(f"Total files: {stats.total_files}")
    stats.compile()
    for codec in stats.codecs.values():
        # if codec.name not in ("JPEG (15%) (Lossy)", "AVIF (15%) (Lossy)"):
        #     continue

        print(f"Per image statistics for codec {codec.name}")
        assert codec.size_bytes.mean is not None
        print("- " + str(codec.size_bytes))
        print("- " + str(codec.relative_sizes))
        print("- " + str(codec.time_mcs))
        if codec.lossy:
            print("Hashes:")
            for metric in codec.hashes.values():
                assert metric.var is not None
                print("- " + str(metric))
            print("Metrics:")
            for metric in codec.metrics.values():
                assert metric.var is not None
                print("- " + str(metric))

        print()


def main():
    from sys import argv

    folder = "logs"
    ignore_zeroes = False
    if len(argv) > 1:
        if "-i" in argv:
            ignore_zeroes = True
        if "-l" in argv:
            folder = argv[argv.index("-l") + 1]

    logs = sorted(Path(folder).glob("*.log"))
    stats: Optional[Status] = None
    for log in logs:
        print("Parsing log file:", log)
        curr_stats = parse_log_file(log, ignore_zeroes)
        if stats is None:
            stats = curr_stats
        else:
            stats = stats + curr_stats

    assert stats is not None
    divide_stats(stats)


main()
