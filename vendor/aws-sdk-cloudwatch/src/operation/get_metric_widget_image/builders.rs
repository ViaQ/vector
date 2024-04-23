// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_metric_widget_image::_get_metric_widget_image_output::GetMetricWidgetImageOutputBuilder;

pub use crate::operation::get_metric_widget_image::_get_metric_widget_image_input::GetMetricWidgetImageInputBuilder;

impl GetMetricWidgetImageInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_metric_widget_image::GetMetricWidgetImageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_metric_widget_image::GetMetricWidgetImageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_metric_widget_image();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetMetricWidgetImage`.
///
/// <p>You can use the <code>GetMetricWidgetImage</code> API to retrieve a snapshot graph of one or more Amazon CloudWatch metrics as a bitmap image. You can then embed this image into your services and products, such as wiki pages, reports, and documents. You could also retrieve images regularly, such as every minute, and create your own custom live dashboard.</p>
/// <p>The graph you retrieve can include all CloudWatch metric graph features, including metric math and horizontal and vertical annotations.</p>
/// <p>There is a limit of 20 transactions per second for this API. Each <code>GetMetricWidgetImage</code> action has the following limits:</p>
/// <ul>
/// <li> <p>As many as 100 metrics in the graph.</p> </li>
/// <li> <p>Up to 100 KB uncompressed payload.</p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetMetricWidgetImageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_metric_widget_image::builders::GetMetricWidgetImageInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_metric_widget_image::GetMetricWidgetImageOutput,
        crate::operation::get_metric_widget_image::GetMetricWidgetImageError,
    > for GetMetricWidgetImageFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_metric_widget_image::GetMetricWidgetImageOutput,
            crate::operation::get_metric_widget_image::GetMetricWidgetImageError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetMetricWidgetImageFluentBuilder {
    /// Creates a new `GetMetricWidgetImage`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetMetricWidgetImage as a reference.
    pub fn as_input(&self) -> &crate::operation::get_metric_widget_image::builders::GetMetricWidgetImageInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_metric_widget_image::GetMetricWidgetImageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_metric_widget_image::GetMetricWidgetImageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_metric_widget_image::GetMetricWidgetImage::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_metric_widget_image::GetMetricWidgetImage::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_metric_widget_image::GetMetricWidgetImageOutput,
        crate::operation::get_metric_widget_image::GetMetricWidgetImageError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>A JSON string that defines the bitmap graph to be retrieved. The string includes the metrics to include in the graph, statistics, annotations, title, axis limits, and so on. You can include only one <code>MetricWidget</code> parameter in each <code>GetMetricWidgetImage</code> call.</p>
    /// <p>For more information about the syntax of <code>MetricWidget</code> see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Metric-Widget-Structure.html">GetMetricWidgetImage: Metric Widget Structure and Syntax</a>.</p>
    /// <p>If any metric on the graph could not load all the requested data points, an orange triangle with an exclamation point appears next to the graph legend.</p>
    pub fn metric_widget(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metric_widget(input.into());
        self
    }
    /// <p>A JSON string that defines the bitmap graph to be retrieved. The string includes the metrics to include in the graph, statistics, annotations, title, axis limits, and so on. You can include only one <code>MetricWidget</code> parameter in each <code>GetMetricWidgetImage</code> call.</p>
    /// <p>For more information about the syntax of <code>MetricWidget</code> see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Metric-Widget-Structure.html">GetMetricWidgetImage: Metric Widget Structure and Syntax</a>.</p>
    /// <p>If any metric on the graph could not load all the requested data points, an orange triangle with an exclamation point appears next to the graph legend.</p>
    pub fn set_metric_widget(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_metric_widget(input);
        self
    }
    /// <p>A JSON string that defines the bitmap graph to be retrieved. The string includes the metrics to include in the graph, statistics, annotations, title, axis limits, and so on. You can include only one <code>MetricWidget</code> parameter in each <code>GetMetricWidgetImage</code> call.</p>
    /// <p>For more information about the syntax of <code>MetricWidget</code> see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Metric-Widget-Structure.html">GetMetricWidgetImage: Metric Widget Structure and Syntax</a>.</p>
    /// <p>If any metric on the graph could not load all the requested data points, an orange triangle with an exclamation point appears next to the graph legend.</p>
    pub fn get_metric_widget(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_metric_widget()
    }
    /// <p>The format of the resulting image. Only PNG images are supported.</p>
    /// <p>The default is <code>png</code>. If you specify <code>png</code>, the API returns an HTTP response with the content-type set to <code>text/xml</code>. The image data is in a <code>MetricWidgetImage</code> field. For example:</p>
    /// <p> <code>
    /// <getmetricwidgetimageresponse xmlns="<URLstring">
    /// &gt;
    /// </getmetricwidgetimageresponse></code> </p>
    /// <p> <code>
    /// <getmetricwidgetimageresult></getmetricwidgetimageresult></code> </p>
    /// <p> <code>
    /// <metricwidgetimage></metricwidgetimage></code> </p>
    /// <p> <code> iVBORw0KGgoAAAANSUhEUgAAAlgAAAGQEAYAAAAip...</code> </p>
    /// <p> <code> </code> </p>
    /// <p> <code> </code> </p>
    /// <p> <code>
    /// <responsemetadata></responsemetadata></code> </p>
    /// <p> <code>
    /// <requestid>
    /// 6f0d4192-4d42-11e8-82c1-f539a07e0e3b
    /// </requestid></code> </p>
    /// <p> <code> </code> </p>
    /// <p> <code></code> </p>
    /// <p>The <code>image/png</code> setting is intended only for custom HTTP requests. For most use cases, and all actions using an Amazon Web Services SDK, you should use <code>png</code>. If you specify <code>image/png</code>, the HTTP response has a content-type set to <code>image/png</code>, and the body of the response is a PNG image. </p>
    pub fn output_format(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.output_format(input.into());
        self
    }
    /// <p>The format of the resulting image. Only PNG images are supported.</p>
    /// <p>The default is <code>png</code>. If you specify <code>png</code>, the API returns an HTTP response with the content-type set to <code>text/xml</code>. The image data is in a <code>MetricWidgetImage</code> field. For example:</p>
    /// <p> <code>
    /// <getmetricwidgetimageresponse xmlns="<URLstring">
    /// &gt;
    /// </getmetricwidgetimageresponse></code> </p>
    /// <p> <code>
    /// <getmetricwidgetimageresult></getmetricwidgetimageresult></code> </p>
    /// <p> <code>
    /// <metricwidgetimage></metricwidgetimage></code> </p>
    /// <p> <code> iVBORw0KGgoAAAANSUhEUgAAAlgAAAGQEAYAAAAip...</code> </p>
    /// <p> <code> </code> </p>
    /// <p> <code> </code> </p>
    /// <p> <code>
    /// <responsemetadata></responsemetadata></code> </p>
    /// <p> <code>
    /// <requestid>
    /// 6f0d4192-4d42-11e8-82c1-f539a07e0e3b
    /// </requestid></code> </p>
    /// <p> <code> </code> </p>
    /// <p> <code></code> </p>
    /// <p>The <code>image/png</code> setting is intended only for custom HTTP requests. For most use cases, and all actions using an Amazon Web Services SDK, you should use <code>png</code>. If you specify <code>image/png</code>, the HTTP response has a content-type set to <code>image/png</code>, and the body of the response is a PNG image. </p>
    pub fn set_output_format(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_output_format(input);
        self
    }
    /// <p>The format of the resulting image. Only PNG images are supported.</p>
    /// <p>The default is <code>png</code>. If you specify <code>png</code>, the API returns an HTTP response with the content-type set to <code>text/xml</code>. The image data is in a <code>MetricWidgetImage</code> field. For example:</p>
    /// <p> <code>
    /// <getmetricwidgetimageresponse xmlns="<URLstring">
    /// &gt;
    /// </getmetricwidgetimageresponse></code> </p>
    /// <p> <code>
    /// <getmetricwidgetimageresult></getmetricwidgetimageresult></code> </p>
    /// <p> <code>
    /// <metricwidgetimage></metricwidgetimage></code> </p>
    /// <p> <code> iVBORw0KGgoAAAANSUhEUgAAAlgAAAGQEAYAAAAip...</code> </p>
    /// <p> <code> </code> </p>
    /// <p> <code> </code> </p>
    /// <p> <code>
    /// <responsemetadata></responsemetadata></code> </p>
    /// <p> <code>
    /// <requestid>
    /// 6f0d4192-4d42-11e8-82c1-f539a07e0e3b
    /// </requestid></code> </p>
    /// <p> <code> </code> </p>
    /// <p> <code></code> </p>
    /// <p>The <code>image/png</code> setting is intended only for custom HTTP requests. For most use cases, and all actions using an Amazon Web Services SDK, you should use <code>png</code>. If you specify <code>image/png</code>, the HTTP response has a content-type set to <code>image/png</code>, and the body of the response is a PNG image. </p>
    pub fn get_output_format(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_output_format()
    }
}
